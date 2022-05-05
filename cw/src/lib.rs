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
    New {
        /// Contract name
        name: String,
        /// Path to store generated contract
        #[clap(short, long)]
        target_dir: Option<PathBuf>,
        /// Template's version, using latest version if not specified (all available versions can be found here: `https://github.com/InterWasm/cw-template/branches`)
        #[clap(short, long)]
        version: Option<String>,
    },
}

impl CW {
    pub fn execute(self: &Self) -> Result<()> {
        match self {
            CW::New {
                name,
                target_dir,
                version,
            } => CW::new(name, version, target_dir),
        }
    }

    fn new(name: &String, version: &Option<String>, target_dir: &Option<PathBuf>) -> Result<()> {
        let version = version.as_ref().map(|v| v.as_str()).unwrap_or(&"main");
        let target_dir = target_dir
            .as_ref()
            .map(|p| p.as_path())
            .unwrap_or(&Path::new("contracts"));

        let target_dir_display = target_dir.display();
        let current_dir = env::current_dir().with_context(|| "Unable to get current directory.")?;
        fs::create_dir_all(target_dir)
            .with_context(|| format!("Unable to create directory: {target_dir_display}"))?;
        env::set_current_dir(Path::new(target_dir)).with_context(|| {
            format!("Unable to set current directory to {target_dir_display}`.")
        })?;

        let CargoGen::Generate(args) = CargoGen::from_iter(
            vec![
                "cargo",
                "generate",
                "InterWasm/cw-template", // TODO: extract to config
                "--branch",
                version,
                "--name",
                name,
            ]
            .iter(),
        );

        cargo_generate(args)
        .with_context(|| format!("Unable to generate contract `{name}` with template branch `{version}` to `{target_dir_display}`."))?;

        env::set_current_dir(current_dir.as_path()).with_context(|| {
            format!("Unable to set current directory back to current directory after changed to `{target_dir_display}`.")
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

        CW::new(&"counter-1".to_string(), &None, &None).unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());

        CW::new(&"counter-2".to_string(), &None, &None).unwrap();
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

        CW::new(&"counter-1".to_string(), &Some("0.16".to_string()), &None).unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());
        assert_version(Path::new("contracts/counter-1/Cargo.toml"), "0.16");

        CW::new(&"counter-2".to_string(), &Some("0.16".to_string()), &None).unwrap();
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

        CW::new(
            &"counter-1".to_string(),
            &None,
            &Some(PathBuf::from_str("custom-path").unwrap()),
        )
        .unwrap();
        temp.child("custom-path/counter-1")
            .assert(predicate::path::exists());

        CW::new(
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
