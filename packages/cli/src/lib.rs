#![doc = r#"
# Beaker

<p align="center">
<a href="https://docs.osmosis.zone/developing/dapps/get_started/">
    <img src="https://github.com/osmosis-labs/beaker/blob/main/assets/beaker.png?raw=true" alt="Beaker logo" title="Beaker" align="center" height="150" />
</a>
</p>

<p align="center" width="100%">
    <img  height="20" src="https://github.com/osmosis-labs/beaker/actions/workflows/doctest.yml/badge.svg">
    <img height="20" src="https://github.com/osmosis-labs/beaker/actions/workflows/lint.yml/badge.svg">
    <a href="https://github.com/osmosis-labs/beaker/blob/main/LICENSE-APACHE"><img height="20" src="https://img.shields.io/badge/license-APACHE-blue.svg"></a>
    <a href="https://github.com/osmosis-labs/beaker/blob/main/LICENSE-MIT"><img height="20" src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
    <a href="https://deps.rs/repo/github/osmosis-labs/beaker"><img height="20" src="https://deps.rs/repo/github/osmosis-labs/beaker/status.svg"></a>
    <a href="https://crates.io/crates/beaker"><img height="20" src="https://img.shields.io/crates/v/beaker.svg"></a>
</p>

[Beaker](https://github.com/osmosis-labs/beaker) makes it easy to scaffold a new cosmwasm app, with all of the dependencies for osmosis hooked up, interactive console, a sample front-end, client generation and more.

- [Getting Started](https://github.com/osmosis-labs/beaker#getting-started)
- [Command Reference](https://github.com/osmosis-labs/beaker/tree/main/docs/commands)
- [Config Reference](https://github.com/osmosis-labs/beaker/tree/main/docs/config)
"#]

mod framework;
mod modules;
mod support;

use anyhow::{Context as _, Result};
use clap::{AppSettings, Parser, Subcommand};
use config::Config;
use data_doc_derive::GetDataDocs;
use modules::{
    key::entrypoint::{KeyCmd, KeyModule},
    task::entrypoint::{TaskCmd, TaskModule},
};
use serde::{Deserialize, Serialize};
use support::node::run_npx;

pub use framework::{
    config::{GlobalConfig, Network, NetworkVariant},
    Context, Module,
};
pub use modules::wasm::{WasmCmd, WasmConfig, WasmModule};
pub use modules::workspace::{WorkspaceCmd, WorkspaceConfig, WorkspaceModule};
pub use support::cosmos::{Client, SigningClient};
pub use support::gas::{Gas, GasPrice};
pub use support::state::{
    Proposal, State, WasmRef, STATE_DIR, STATE_FILE_LOCAL, STATE_FILE_SHARED,
};

use crate::modules::{key::config::KeyConfig, task::config::TaskConfig};

#[derive(Parser)]
#[clap(author, version,about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Cli {
    // config: Option<PathBuf>,
    #[clap(subcommand)]
    pub command: Commands,
}

// === APP DEFINITION ===
// Could potentially move all this to macro
#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Commands {
    #[clap(flatten)]
    Workspace(WorkspaceCmd),
    /// Manipulating and interacting with CosmWasm contract
    Wasm {
        #[clap(subcommand)]
        cmd: WasmCmd,
    },
    /// Managing key backed by system's secret store
    Key {
        #[clap(subcommand)]
        cmd: KeyCmd,
    },
    /// Launch interactive console for interacting with the project
    Console {
        #[clap(short, long, default_value = "local")]
        network: String,
    },
    /// Managing tasks for the project
    Task {
        #[clap(subcommand)]
        cmd: TaskCmd,
    },
}

#[derive(Serialize, Deserialize, GetDataDocs)]
pub struct ConsoleConfig {
    /// Put all accounts under `account` namespace in console context if set true.
    /// Otherwise, they will be available in global namespace
    account_namespace: bool,

    /// Put all contracts under `contract` namespace in console context if set true.
    /// Otherwise, they will be available in global namespace
    contract_namespace: bool,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        Self {
            account_namespace: true,
            contract_namespace: true,
        }
    }
}

fn console(network: &str) -> Result<()> {
    let console_ctx = ConsoleContext::new();
    let wasm_ctx = WasmContext::new();
    let workspace_ctx = WorkspaceContext::new();

    let conf = serde_json::json!({
        "global": console_ctx.global_config()?,
        "wasm": wasm_ctx.config()?,
        "workspace": workspace_ctx.config()?,
        "console": console_ctx.config()?
    });

    run_npx(
        [
            beaker_console().as_str(),
            console_ctx.root()?.to_str().unwrap(),
            network,
            serde_json::to_string(&conf)?.as_str(),
        ],
        "beaker-console error",
    )
}

#[cfg(debug_assertions)]
fn beaker_console() -> String {
    "beaker-console".to_string()
}

#[cfg(not(debug_assertions))]
fn beaker_console() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("beaker-console@{}", version)
}

context!(
    WasmContext, config = { wasm: WasmConfig };
    WorkspaceContext, config = { workspace: WorkspaceConfig };
    ConsoleContext, config = { console: ConsoleConfig };
    KeyContext, config = { key: KeyConfig };
    TaskContext, config = { task: TaskConfig }
);

pub fn execute(cmd: &Commands) -> Result<()> {
    match cmd {
        Commands::Wasm { cmd } => WasmModule::execute(WasmContext::new(), cmd),
        Commands::Workspace(cmd) => WorkspaceModule::execute(WorkspaceContext::new(), cmd),
        Commands::Console { network } => console(network),
        Commands::Key { cmd } => KeyModule::execute(KeyContext::new(), cmd),
        Commands::Task { cmd } => TaskModule::execute(TaskContext::new(), cmd),
    }
}

#[cfg(test)]
mod tests {
    use assert_fs::{assert::PathAssert, fixture::PathChild, TempDir};
    use predicates::prelude::predicate;
    use serial_test::serial;
    use std::{env, fs, path::Path};

    use super::*;

    fn setup() -> TempDir {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(&temp).unwrap();
        temp
    }
    #[test]
    #[serial]
    fn test_configuration() {
        let temp = setup();
        execute(&Commands::Workspace(WorkspaceCmd::New {
            name: "dapp".to_string(),
            target_dir: None,
            branch: None,
        }))
        .unwrap();

        let mut path = temp.to_path_buf();
        path.push(Path::new("dapp"));
        env::set_current_dir(&path).unwrap();

        let conf = r#"
[wasm]
contract_dir = "whatever""#;

        path.push(Path::new("Beaker.toml"));
        fs::write(path.as_path(), conf).unwrap();

        execute(&Commands::Wasm {
            cmd: WasmCmd::New {
                contract_name: "counter".to_string(),
                target_dir: None,
                version: None,
            },
        })
        .unwrap();

        temp.child("dapp/Beaker.toml")
            .assert(predicate::path::exists());

        temp.child("dapp/whatever/counter/Cargo.toml")
            .assert(predicate::path::exists());
    }
}
