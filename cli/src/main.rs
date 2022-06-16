mod framework;
mod modules;
mod support;

use anyhow::{bail, Context as _, Result};
use clap::{AppSettings, Parser, Subcommand};
use config::Config;
use framework::{Context, Module};
use modules::wasm::{WasmCmd, WasmConfig, WasmModule};
use modules::workspace::{WorkspaceCmd, WorkspaceConfig, WorkspaceModule};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[clap(author, version,about, long_about = None)]
#[clap(propagate_version = true)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Cli {
    config: Option<PathBuf>,
    #[clap(subcommand)]
    command: Commands,
}

// === APP DEFINITION ===
// Could potentially move all this to macro
#[derive(Subcommand)]
pub enum Commands {
    #[clap(flatten)]
    Workspace(WorkspaceCmd),
    /// Manipulating and interacting with CosmWasm contract
    Wasm {
        #[clap(subcommand)]
        cmd: WasmCmd,
    },
    /// Launch interactive console for interacting with the project
    Console {
        #[clap(short, long, default_value = "local")]
        network: String,
    },
}

#[derive(Serialize, Deserialize)]
struct ConsoleConfig {
    /// Set account namespace in console context if set true.
    /// All accounts will be available in console context if set false
    account_namespace: bool,

    /// Set contract namespace in console context if set true.
    /// All contracts will be available in console context if set false
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

    let npx = Command::new("npx")
        .arg("beaker-console")
        .arg(console_ctx.root()?.as_os_str())
        .arg(network)
        .arg(serde_json::to_string(&conf)?)
        .spawn();

    match npx {
        Ok(mut npx) => {
            npx.wait()?;
            Ok(())
        }
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                bail!("`npx`, which pre-bundled with npm >= 5.2.0, is required for console but not found. Please install `npm` or check your path.")
            } else {
                Err(e).with_context(|| "beaker-console error")
            }
        }
    }
}

context!(
    WasmContext, config = { wasm: WasmConfig };
    WorkspaceContext, config = { workspace: WorkspaceConfig };
    ConsoleContext, config = { console: ConsoleConfig }
);

pub fn execute(cmd: &Commands) -> Result<()> {
    match cmd {
        Commands::Wasm { cmd } => WasmModule::execute(WasmContext::new(), cmd),
        Commands::Workspace(cmd) => WorkspaceModule::execute(WorkspaceContext::new(), cmd),
        Commands::Console { network } => console(network),
    }
}

// === END APP DEFINITION ===

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    execute(&cli.command)
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
