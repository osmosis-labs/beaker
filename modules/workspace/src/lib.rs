use anyhow::Result;
use clap::Subcommand;
use derive_new::new;
use protostar_core::Module;
use protostar_helper_template::Template;
use serde::Deserialize;
use serde::Serialize;

use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct WorkspaceConfig {
    // TODO: add config file name
    template: Template,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            template: Template::new(
                "workspace-template".to_string(),
                "iboss-ptk/protostar-sdk".to_string(),
                "main".to_string(),
                PathBuf::from("."),
                Some("templates/project".to_string()),
            ),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum WorkspaceCmd {
    /// create new workspace from boilerplate
    New {
        /// workspace name
        name: String,
        /// path to store generated workspace
        #[clap(short, long)]
        target_dir: Option<PathBuf>,
        /// template's branch, using main if not specified
        #[clap(short, long)]
        branch: Option<String>,
    },
}

#[derive(new)]
pub struct WorkspaceModule {}

impl WorkspaceModule {
    pub fn new_(
        cfg: &WorkspaceConfig,
        name: &String,
        branch: &Option<String>,
        target_dir: &Option<PathBuf>,
    ) -> Result<()> {
        cfg.template
            .with_name(Some(name.to_string()))
            .with_branch(branch.to_owned())
            .with_target_dir(target_dir.to_owned())
            .generate()
    }
}
impl Module<'_, WorkspaceConfig, WorkspaceCmd, anyhow::Error> for WorkspaceModule {
    // fn execute_(
    //     self: &Self,
    //     ctx: Box<dyn Context<'a, CWConfig>>,
    //     cmd: &WorkspaceCmd,
    // ) -> Result<()> {
    //     let cfg = ctx.config()?;
    //     match cmd {
    //         WorkspaceCmd::New {
    //             name,
    //             target_dir,
    //             branch,
    //         } => WorkspaceModule::new_(&cfg, &name, &branch, &target_dir),
    //     }
    // }
    fn execute(self: &Self, cfg: &WorkspaceConfig, cmd: &WorkspaceCmd) -> Result<()> {
        match cmd {
            WorkspaceCmd::New {
                name,
                target_dir,
                branch,
            } => WorkspaceModule::new_(&cfg, &name, &branch, &target_dir),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use predicates::prelude::*;
    use serial_test::serial;
    use std::{env, str::FromStr};

    #[test]
    #[serial]
    fn generate_project_with_default_path() {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(temp.to_path_buf()).unwrap();

        temp.child("cosmwasm-dapp")
            .assert(predicate::path::missing());

        let cfg = WorkspaceConfig::default();
        WorkspaceModule::new()
            .execute(
                &cfg,
                &WorkspaceCmd::New {
                    name: "cosmwasm-dapp".to_string(),
                    target_dir: None,
                    branch: None,
                },
            )
            .unwrap();

        temp.child("cosmwasm-dapp/Protostar.toml")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }

    #[test]
    #[serial]
    fn generate_project_with_custom_path() {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(temp.to_path_buf()).unwrap();

        temp.child("custom-path").assert(predicate::path::missing());

        temp.child("custom-path/cosmwasm-dapp")
            .assert(predicate::path::missing());

        let cfg = WorkspaceConfig::default();
        WorkspaceModule::new()
            .execute(
                &cfg,
                &WorkspaceCmd::New {
                    name: "cosmwasm-dapp".to_string(),
                    target_dir: Some(PathBuf::from_str("custom-path").unwrap()),
                    branch: None,
                },
            )
            .unwrap();

        temp.child("custom-path/cosmwasm-dapp/Protostar.toml")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }
}
