use anyhow::Result;
use clap::Subcommand;
use protostar_helper_template::Template;
use std::path::Path;
use std::path::PathBuf;

pub struct Project {
    repo: String,
    subfolder: String,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            repo: "iboss-ptk/protostar-sdk".to_string(),
            subfolder: "templates/project".to_string(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// generate CosmWasm contract from boilerplate
    New {
        /// contract name
        name: String,
        /// path to store generated contract
        #[clap(short, long)]
        target_dir: Option<PathBuf>,
        /// template's version, using latest version if not specified (all available versions can be found here: `https://github.com/InterWasm/cw-template/branches`)
        #[clap(short, long)]
        version: Option<String>,
    },
}

impl Cmd {
    fn new(
        name: &String,
        repo: &str,
        subfolder: &str,
        version: &Option<String>,
        target_dir: &Option<PathBuf>,
    ) -> Result<()> {
        let version = version.as_ref().map(|v| v.as_str()).unwrap_or(&"main");
        let target_dir = target_dir
            .as_ref()
            .map(|p| p.as_path())
            .unwrap_or(&Path::new("."));

        Template::new(name, repo, version, target_dir)
            .with_subfolder(subfolder)
            .generate()
    }
}

impl Project {
    pub fn new(repo: String, subfolder: String) -> Self {
        Project { repo, subfolder }
    }
    pub fn execute(self: &Self, cmd: Cmd) -> Result<()> {
        match cmd {
            Cmd::New {
                name,
                target_dir,
                version,
            } => Cmd::new(&name, &self.repo, &self.subfolder, &version, &target_dir),
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

        Project::default()
            .execute(Cmd::New {
                name: "cosmwasm-dapp".to_string(),
                target_dir: None,
                version: None,
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

        Project::default()
            .execute(Cmd::New {
                name: "cosmwasm-dapp".to_string(),
                target_dir: Some(PathBuf::from_str("custom-path").unwrap()),
                version: None,
            })
            .unwrap();

        temp.child("custom-path/cosmwasm-dapp/Protostar.toml")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }
}
