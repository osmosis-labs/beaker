use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use config::Config;
use protostar_cw::CW;
use protostar_workspace::Workspace;
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
    /// Manipulating and interacting with Protostar project
    Workspace {
        #[clap(subcommand)]
        cmd: protostar_workspace::Cmd,
    },
    /// Manipulating and interacting with CosmWasm contract
    CW {
        #[clap(subcommand)]
        cmd: protostar_cw::Cmd,
    },
}

#[derive(Default, Serialize, Deserialize)]
pub struct App {
    cw: CW,
    workspace: Workspace,
}

impl App {
    pub fn with_config(file_name: &str) -> Result<App> {
        let conf = Config::builder()
            .add_source(Config::try_from(&Self::default())?)
            .add_source(config::File::with_name(file_name))
            .build()?;

        conf.try_deserialize::<App>()
            .with_context(|| "Unable to deserilize configuration.")
    }

    pub fn execute(self: &Self, cmd: &Commands) -> Result<()> {
        match cmd {
            Commands::CW { cmd } => self.cw.execute(cmd),
            Commands::Workspace { cmd } => self.workspace.execute(cmd),
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let app = App {
        cw: CW::default(),
        workspace: Workspace::default(),
    };

    app.execute(&cli.command)
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

        let app = App::default();

        app.execute(&Commands::Workspace {
            cmd: protostar_workspace::Cmd::New {
                name: "dapp".to_string(),
                target_dir: None,
                version: None,
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

        let app = App::with_config("Protostar.toml").unwrap();

        app.execute(&Commands::CW {
            cmd: protostar_cw::Cmd::New {
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
