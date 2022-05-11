use anyhow::Context as ErrContext;
use anyhow::Result;
use clap::Subcommand;
use derive_new::new;
use protostar_core::Context;
use protostar_core::Module;
use protostar_helper_template::Template;
use serde::Deserialize;
use serde::Serialize;

use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct WorkspaceConfig {
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
    /// Create new workspace from boilerplate
    New {
        /// Workspace name
        name: String,
        /// Path to store generated workspace
        #[clap(short, long)]
        target_dir: Option<PathBuf>,
        /// Template's branch, using main if not specified
        #[clap(short, long)]
        branch: Option<String>,
    },
}

#[derive(new)]
pub struct WorkspaceModule {}

impl<'a> WorkspaceModule {
    pub fn new_<Ctx: Context<'a, WorkspaceConfig>>(
        ctx: Ctx,
        name: &String,
        branch: &Option<String>,
        target_dir: &Option<PathBuf>,
    ) -> Result<()> {
        let template = ctx
            .config()?
            .template
            .with_name(Some(name.to_string()))
            .with_branch(branch.to_owned())
            .with_target_dir(target_dir.to_owned());

        template.generate()?;

        let root_dir = template.target_dir().join::<PathBuf>(name.into());

        let default_config_file_name = "Protostar.toml";
        let config_file_name = ctx.config_file_name();
        fs::rename(
            root_dir.join::<PathBuf>(default_config_file_name.into()),
            root_dir.join::<PathBuf>(config_file_name.clone().into()),
        )
        .with_context(|| {
            format!("Unable to rename {default_config_file_name} to {config_file_name}")
        })
    }
}
impl<'a> Module<'a, WorkspaceConfig, WorkspaceCmd, anyhow::Error> for WorkspaceModule {
    fn execute<Ctx: Context<'a, WorkspaceConfig>>(
        ctx: Ctx,
        cmd: &WorkspaceCmd,
    ) -> Result<(), anyhow::Error> {
        match cmd {
            WorkspaceCmd::New {
                name,
                target_dir,
                branch,
            } => WorkspaceModule::new_(ctx, name, branch, target_dir),
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

    struct WorkspaceContext {}
    impl<'a> Context<'a, WorkspaceConfig> for WorkspaceContext {}

    struct MembraneWorkspaceContext {}
    impl<'a> Context<'a, WorkspaceConfig> for MembraneWorkspaceContext {
        fn config_file_name(&self) -> String {
            String::from("Membrane.toml")
        }
    }

    #[test]
    #[serial]
    fn generate_project_with_default_path() {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(&temp).unwrap();

        temp.child("cosmwasm-dapp")
            .assert(predicate::path::missing());

        WorkspaceModule::execute(
            WorkspaceContext {},
            &WorkspaceCmd::New {
                name: "cosmwasm-dapp".to_string(),
                target_dir: None,
                branch: None,
            },
        )
        .unwrap();

        temp.child("cosmwasm-dapp/Protostar.toml")
            .assert(predicate::path::exists());

        // with custom config file name
        WorkspaceModule::execute(
            MembraneWorkspaceContext {},
            &WorkspaceCmd::New {
                name: "osmo-dapp".to_string(),
                target_dir: None,
                branch: None,
            },
        )
        .unwrap();

        temp.child("osmo-dapp/Membrane.toml")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }

    #[test]
    #[serial]
    fn generate_project_with_custom_path() {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(&temp).unwrap();

        temp.child("custom-path").assert(predicate::path::missing());

        temp.child("custom-path/cosmwasm-dapp")
            .assert(predicate::path::missing());

        WorkspaceModule::execute(
            WorkspaceContext {},
            &WorkspaceCmd::New {
                name: "cosmwasm-dapp".to_string(),
                target_dir: Some(PathBuf::from_str("custom-path").unwrap()),
                branch: None,
            },
        )
        .unwrap();

        temp.child("custom-path/cosmwasm-dapp/Protostar.toml")
            .assert(predicate::path::exists());

        // with custom config file name
        WorkspaceModule::execute(
            MembraneWorkspaceContext {},
            &WorkspaceCmd::New {
                name: "osmo-dapp".to_string(),
                target_dir: Some(PathBuf::from_str("custom-path").unwrap()),
                branch: None,
            },
        )
        .unwrap();

        temp.child("custom-path/osmo-dapp/Membrane.toml")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }
}
