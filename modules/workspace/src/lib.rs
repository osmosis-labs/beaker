use anyhow::Result;
use clap::Subcommand;
use protostar_helper_template::Template;
use serde::Deserialize;
use serde::Serialize;

use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    // TODO: add config file name
    template: Template,
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            template: Template::new(
                "workspace-template".to_string(),
                "iboss-ptk/protostar-sdk".to_string(),
                "main".to_string(),
                PathBuf::from("."),
            )
            .with_subfolder("templates/project"),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
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

impl Workspace {
    pub fn new(repo: &str, subfolder: &str) -> Self {
        let w = Workspace::default();
        let t = w.template;
        Workspace {
            template: Template::new(
                t.name().to_string(),
                repo.to_string(),
                t.branch().to_string(),
                t.target_dir().to_owned(),
            )
            .with_subfolder(&subfolder.clone()),
        }
    }
    pub fn execute(self: &Self, cmd: &Cmd) -> Result<()> {
        match cmd {
            Cmd::New {
                name,
                target_dir,
                branch,
            } => self.new_workspace(&name, &branch, &target_dir),
        }
    }

    pub fn new_workspace(
        &self,
        name: &String,
        branch: &Option<String>,
        target_dir: &Option<PathBuf>,
    ) -> Result<()> {
        let branch = branch
            .as_ref()
            .map(|v| v.as_str())
            .unwrap_or(&self.template.branch());
        let target_dir = target_dir
            .as_ref()
            .unwrap_or(self.template.target_dir())
            .to_owned();

        let template = Template::new(
            name.to_string(),
            self.template.repo().to_string(),
            branch.to_string(),
            target_dir,
        );
        let template = match &self.template.subfolder() {
            Some(subfolder) => template.with_subfolder(subfolder),
            _ => template,
        };
        template.generate()
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

        Workspace::default()
            .execute(&Cmd::New {
                name: "cosmwasm-dapp".to_string(),
                target_dir: None,
                branch: None,
            })
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

        Workspace::default()
            .execute(&Cmd::New {
                name: "cosmwasm-dapp".to_string(),
                target_dir: Some(PathBuf::from_str("custom-path").unwrap()),
                branch: None,
            })
            .unwrap();

        temp.child("custom-path/cosmwasm-dapp/Protostar.toml")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }
}
