use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use config::Config;
use protostar_core::Module;
use protostar_cw::{CWConfig, CWModule};
use protostar_workspace::{WorkspaceConfig, WorkspaceModule};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[clap(author, version,about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    config: Option<PathBuf>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manipulating and interacting with the workspace
    Workspace {
        #[clap(subcommand)]
        cmd: protostar_workspace::WorkspaceCmd,
    },
    /// Manipulating and interacting with CosmWasm contract
    CW {
        #[clap(subcommand)]
        cmd: protostar_cw::CWCmd,
    },
}

#[derive(Default, Serialize, Deserialize)]
pub struct AppConfig {
    cw: CWConfig,
    workspace: WorkspaceConfig,
}

impl AppConfig {
    pub fn with_config(file_name: &str) -> Result<AppConfig> {
        let conf = Config::builder()
            .add_source(Config::try_from(&Self::default())?)
            .add_source(config::File::with_name(file_name))
            .build()?;

        conf.try_deserialize::<AppConfig>()
            .with_context(|| "Unable to deserilize configuration.")
    }
}

pub fn execute(cfg: &AppConfig, cmd: &Commands) -> Result<()> {
    match cmd {
        Commands::CW { cmd } => CWModule::new().execute(&cfg.cw, &cmd),
        Commands::Workspace { cmd } => WorkspaceModule::new().execute(&cfg.workspace, &cmd),
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let app_cfg = AppConfig {
        cw: CWConfig::default(),
        workspace: WorkspaceConfig::default(),
    };

    execute(&app_cfg, &cli.command)
}
#[cfg(test)]
mod tests {
    use std::{env, fs, path::Path};

    use assert_fs::{assert::PathAssert, fixture::PathChild};
    use predicates::prelude::predicate;

    use super::*;

    #[test]
    fn test_configuration() {
        let temp = assert_fs::TempDir::new().unwrap();

        env::set_current_dir(temp.to_path_buf()).unwrap();

        let app = AppConfig::default();

        execute(
            &app,
            &Commands::Workspace {
                cmd: protostar_workspace::WorkspaceCmd::New {
                    name: "dapp".to_string(),
                    target_dir: None,
                    branch: None,
                },
            },
        )
        .unwrap();

        let mut path = temp.to_path_buf();
        path.push(Path::new("dapp"));
        env::set_current_dir(path.to_path_buf()).unwrap();

        let conf = r#"
[cw]
contract_dir = "whatever""#;

        path.push(Path::new("Protostar.toml"));
        fs::write(path.as_path(), conf).unwrap();

        let app = AppConfig::with_config("Protostar.toml").unwrap();

        execute(
            &app,
            &Commands::CW {
                cmd: protostar_cw::CWCmd::New {
                    name: "counter".to_string(),
                    target_dir: None,
                    version: None,
                },
            },
        )
        .unwrap();

        temp.child("dapp/Protostar.toml")
            .assert(predicate::path::exists());

        temp.child("dapp/whatever/counter/Cargo.toml")
            .assert(predicate::path::exists());
    }
}
