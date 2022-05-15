mod workspace;

use anyhow::{Context as ErrContext, Result};
use clap::{AppSettings, Parser, Subcommand};
use config::Config;
use protostar_core::{context, Context, Module};
use protostar_cw::{CWConfig, CWModule};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use workspace::{WorkspaceConfig, WorkspaceModule};

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
    Workspace(workspace::WorkspaceCmd),
    /// Manipulating and interacting with CosmWasm contract
    CW {
        #[clap(subcommand)]
        cmd: protostar_cw::CWCmd,
    },
}

context!(
    CWContext, config = { cw: CWConfig };
    WorkspaceContext, config = { workspace: WorkspaceConfig }
);

pub fn execute(cmd: &Commands) -> Result<()> {
    match cmd {
        Commands::CW { cmd } => CWModule::execute(CWContext {}, cmd),
        Commands::Workspace(cmd) => WorkspaceModule::execute(WorkspaceContext {}, cmd),
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
    use workspace::WorkspaceCmd;

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
[cw]
contract_dir = "whatever""#;

        path.push(Path::new("Protostar.toml"));
        fs::write(path.as_path(), conf).unwrap();

        execute(&Commands::CW {
            cmd: protostar_cw::CWCmd::New {
                name: "counter".to_string(),
                target_dir: None,
                version: None,
            },
        })
        .unwrap();

        temp.child("dapp/Protostar.toml")
            .assert(predicate::path::exists());

        temp.child("dapp/whatever/counter/Cargo.toml")
            .assert(predicate::path::exists());
    }
}
