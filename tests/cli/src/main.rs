use std::path::PathBuf;

use anyhow::{Context as ErrContext, Result};
use clap::{Parser, Subcommand};
use config::Config;
use protostar_core::{Context, Module};
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

struct CWContext {}
impl<'a> Context<'a, CWConfig> for CWContext {
    fn config(&self) -> Result<CWConfig> {
        #[derive(Default, Serialize, Deserialize)]
        struct ConfigWrapper {
            cw: CWConfig,
        }

        let conf = Config::builder().add_source(Config::try_from(&ConfigWrapper::default())?);
        let conf = match self.config_file_path() {
            Ok(path) => conf.add_source(config::File::from(path)),
            _ => conf,
        };
        conf.build()?
            .try_deserialize::<ConfigWrapper>()
            .with_context(|| "Unable to deserilize configuration.")
            .map(|w| w.cw)
    }
}
struct WorkspaceContext {}
impl<'a> Context<'a, WorkspaceConfig> for WorkspaceContext {
    fn config(&self) -> Result<WorkspaceConfig> {
        #[derive(Default, Serialize, Deserialize)]
        struct ConfigWrapper {
            workspace: WorkspaceConfig,
        }

        let conf = Config::builder().add_source(Config::try_from(&ConfigWrapper::default())?);
        let conf = match self.config_file_path() {
            Ok(path) => conf.add_source(config::File::from(path)),
            _ => conf,
        };
        conf.build()?
            .try_deserialize::<ConfigWrapper>()
            .with_context(|| "Unable to deserilize configuration.")
            .map(|w| w.workspace)
    }
}

pub fn execute(cmd: &Commands) -> Result<()> {
    match cmd {
        Commands::CW { cmd } => CWModule::execute_(CWContext {}, &cmd),
        Commands::Workspace { cmd } => WorkspaceModule::execute_(WorkspaceContext {}, &cmd),
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    execute(&cli.command)
}

#[cfg(test)]
mod tests {
    use std::{env, fs, path::Path};

    use assert_fs::{assert::PathAssert, fixture::PathChild, TempDir};
    use predicates::prelude::predicate;

    use super::*;

    fn setup() -> TempDir {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(temp.to_path_buf()).unwrap();
        temp
    }
    #[test]
    fn test_configuration() {
        let temp = setup();
        execute(&Commands::Workspace {
            cmd: protostar_workspace::WorkspaceCmd::New {
                name: "dapp".to_string(),
                target_dir: None,
                branch: None,
            },
        })
        .unwrap();

        let mut path = temp.to_path_buf();
        path.push(Path::new("dapp"));
        env::set_current_dir(path.to_path_buf()).unwrap();

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
