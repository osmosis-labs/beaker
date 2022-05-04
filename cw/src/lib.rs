use anyhow::Context;
use anyhow::Result;
use cargo_generate::{generate as cargo_generate, Cli as CargoGen};
use clap::Subcommand;

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Subcommand, Debug)]
pub enum CW {
    /// Generate CosmWasm contract from boilerplate
    Gen {
        /// Contract name
        name: String,
        /// Template's version, using latest version if not specified (all available versions can be found here: `https://github.com/InterWasm/cw-template/branches`)
        #[clap(short, long)]
        version: Option<String>,
        /// Path to store generated contract
        #[clap(short, long)]
        path: Option<PathBuf>,
    },
}

impl CW {
    pub fn execute(self: &Self) -> Result<()> {
        match self {
            CW::Gen {
                name,
                version,
                path,
            } => CW::gen(name, version, path),
        }
    }

    fn gen(name: &String, version: &Option<String>, path: &Option<PathBuf>) -> Result<()> {
        let default_path: &Path = Path::new("contracts"); // TODO: extract to config
        let target_dir = path.as_ref().map(|p| p.as_path()).unwrap_or(&default_path);
        let current_dir = env::current_dir()?;

        fs::create_dir_all(target_dir)?;
        env::set_current_dir(Path::new(target_dir))?;

        let CargoGen::Generate(args) = CargoGen::from_iter(
            vec![
                "cargo",
                "generate",
                "--git",
                "https://github.com/CosmWasm/cw-template.git", // TODO: extract to config
                "--branch",
                version.as_ref().map(|v| v.as_str()).unwrap_or(&"main"),
                "--name",
                name,
            ]
            .iter(),
        );

        cargo_generate(args)?;

        let target_dir_display = target_dir.display();
        env::set_current_dir(current_dir.as_path()).with_context(|| {
            format!("Could not change directory back to current directory after changed to `{target_dir_display}`")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use cargo_toml::{Dependency, DependencyDetail, Manifest};
    use predicates::prelude::*;
    use serial_test::serial;
    use std::{env, str::FromStr};

    #[test]
    #[serial]
    fn generate_contract_with_default_version_and_path() {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(temp.to_path_buf()).unwrap();

        temp.child("contracts").assert(predicate::path::missing());
        temp.child("contracts/counter-1")
            .assert(predicate::path::missing());
        temp.child("contracts/counter-2")
            .assert(predicate::path::missing());

        CW::gen(&"counter-1".to_string(), &None, &None).unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());

        CW::gen(&"counter-2".to_string(), &None, &None).unwrap();
        temp.child("contracts/counter-2")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }

    #[test]
    #[serial]
    fn generate_contract_with_custom_version() {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(temp.to_path_buf()).unwrap();

        temp.child("contracts").assert(predicate::path::missing());
        temp.child("contracts/counter-1")
            .assert(predicate::path::missing());
        temp.child("contracts/counter-2")
            .assert(predicate::path::missing());

        CW::gen(&"counter-1".to_string(), &Some("0.16".to_string()), &None).unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());
        assert_version(Path::new("contracts/counter-1/Cargo.toml"), "0.16");

        CW::gen(&"counter-2".to_string(), &Some("0.16".to_string()), &None).unwrap();
        temp.child("contracts/counter-2")
            .assert(predicate::path::exists());
        assert_version(Path::new("contracts/counter-2/Cargo.toml"), "0.16");

        temp.close().unwrap();
    }

    #[test]
    #[serial]
    fn generate_contract_with_custom_path() {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(temp.to_path_buf()).unwrap();

        temp.child("custom-path").assert(predicate::path::missing());
        temp.child("custom-path/counter-1")
            .assert(predicate::path::missing());
        temp.child("custom-path/counter-2")
            .assert(predicate::path::missing());

        CW::gen(
            &"counter-1".to_string(),
            &None,
            &Some(PathBuf::from_str("custom-path").unwrap()),
        )
        .unwrap();
        temp.child("custom-path/counter-1")
            .assert(predicate::path::exists());

        CW::gen(
            &"counter-2".to_string(),
            &None,
            &Some(PathBuf::from_str("custom-path").unwrap()),
        )
        .unwrap();
        temp.child("custom-path/counter-2")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }

    fn assert_version(cargo_toml_path: &Path, expected_version: &str) {
        let manifest = Manifest::from_path(cargo_toml_path).unwrap();
        let version = {
            if let Dependency::Detailed(DependencyDetail {
                version: Some(version),
                ..
            }) = manifest.dependencies.get("cosmwasm-std").unwrap()
            {
                version
            } else {
                ""
            }
        };

        assert!(version.starts_with(expected_version))
    }
}
