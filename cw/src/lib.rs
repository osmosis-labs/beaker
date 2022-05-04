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
        /// Path to store generated contract
        #[clap(short, long)]
        path: Option<PathBuf>,
    },
}

impl CW {
    pub fn execute(self: &Self) -> Result<()> {
        match self {
            CW::Gen { name, path } => CW::gen(name, path),
        }
    }

    fn gen(name: &String, path: &Option<PathBuf>) -> Result<()> {
        let default_path: &Path = Path::new("contracts"); // TODO: extract to config
        let target_dir = path.as_ref().map(|p| p.as_path()).unwrap_or(&default_path);
        let current_dir = env::current_dir()?;

        fs::create_dir_all(target_dir)?;
        env::set_current_dir(Path::new(target_dir))?;

        // TODO: add branch / version option
        let CargoGen::Generate(args) = CargoGen::from_iter(
            vec![
                "cargo",
                "generate",
                "--git",
                "https://github.com/CosmWasm/cw-template.git", // TODO: extract to config
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
    use predicates::prelude::*;
    use serial_test::serial;
    use std::{env, str::FromStr};

    #[test]
    #[serial]
    fn generate_contract_with_default_path() {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(temp.to_path_buf()).unwrap();

        temp.child("contracts").assert(predicate::path::missing());
        temp.child("contracts/counter-1")
            .assert(predicate::path::missing());
        temp.child("contracts/counter-2")
            .assert(predicate::path::missing());

        CW::gen(&"counter-1".to_string(), &None).unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());

        CW::gen(&"counter-2".to_string(), &None).unwrap();
        temp.child("contracts/counter-2")
            .assert(predicate::path::exists());

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
            &Some(PathBuf::from_str("custom-path").unwrap()),
        )
        .unwrap();
        temp.child("custom-path/counter-1")
            .assert(predicate::path::exists());

        CW::gen(
            &"counter-2".to_string(),
            &Some(PathBuf::from_str("custom-path").unwrap()),
        )
        .unwrap();
        temp.child("custom-path/counter-2")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }
}
