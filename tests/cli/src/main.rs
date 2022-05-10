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

// - module contains context
// - context can materialize config
// - execute consume context and config (or better yet, config is in context)
// - fn context with default (for the case where you wanna custom config from code)

// TODO:
// [x] 1. Create context trait, impl in `core`
// [x] 2. build root and config fn for the config type
// [ ] 3. strip down cfg from execute function, use it from context instead, pass context to `Module::new`
//   [ ] 3.1. Make Context generic for all modul:::: // Reconstruct Cfg as a wrapper struct for module specific one ..... (on this,)
//     [ ] 3.1.1. Change module interface to support passing in CTX. make sure all the shits still works fine
// [ ] 4. remove app config
// [ ] 5. make workspace module update Protostar.toml to eg. `Membrane.toml` as per config

struct AppContext;

impl AppContext {
    fn new() -> Self {
        AppContext {}
    }
}

impl Context<'_, AppConfig> for AppContext {}

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

pub fn execute(cfg: &AppConfig, cmd: &Commands) -> Result<()> {
    match cmd {
        Commands::CW { cmd } => CWModule::new().execute(&cfg.cw, &cmd),
        Commands::Workspace { cmd } => WorkspaceModule::new().execute(&cfg.workspace, &cmd),
    }
}

fn main() -> Result<(), anyhow::Error> {
    let app_ctx = AppContext::new();
    let app_cfg = app_ctx.config()?;

    let cli = Cli::parse();

    execute(&app_cfg, &cli.command)
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
