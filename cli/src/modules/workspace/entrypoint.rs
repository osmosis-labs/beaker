use super::config::WorkspaceConfig;
use super::ops;
use crate::framework::{Context, Module};
use anyhow::Result;
use clap::Subcommand;
use derive_new::new;
use std::path::PathBuf;

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
            } => ops::new(ctx, name, branch, target_dir),
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

    struct RandomWorkspaceContext {}
    impl<'a> Context<'a, WorkspaceConfig> for RandomWorkspaceContext {
        fn config_file_name(&self) -> String {
            String::from("Random.toml")
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

        temp.child("cosmwasm-dapp/Beaker.toml")
            .assert(predicate::path::exists());

        // with custom config file name
        WorkspaceModule::execute(
            RandomWorkspaceContext {},
            &WorkspaceCmd::New {
                name: "osmo-dapp".to_string(),
                target_dir: None,
                branch: None,
            },
        )
        .unwrap();

        temp.child("osmo-dapp/Random.toml")
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

        temp.child("custom-path/cosmwasm-dapp/Beaker.toml")
            .assert(predicate::path::exists());

        // with custom config file name
        WorkspaceModule::execute(
            RandomWorkspaceContext {},
            &WorkspaceCmd::New {
                name: "osmo-dapp".to_string(),
                target_dir: Some(PathBuf::from_str("custom-path").unwrap()),
                branch: None,
            },
        )
        .unwrap();

        temp.child("custom-path/osmo-dapp/Random.toml")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }
}
