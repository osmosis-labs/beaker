use anyhow::Result;
use clap::Subcommand;
use derive_new::new;
use protostar_core::Context;
use protostar_core::Module;
use protostar_helper_template::Template;
use serde::Deserialize;
use serde::Serialize;

use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct CWConfig {
    pub contract_dir: String,
    pub template_repo: String,
}

impl Default for CWConfig {
    fn default() -> Self {
        Self {
            contract_dir: "contracts".to_string(),
            template_repo: "InterWasm/cw-template".to_string(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum CWCmd {
    /// Create new CosmWasm contract from boilerplate
    New {
        /// Contract name
        name: String,
        /// Path to store generated contract
        #[clap(short, long)]
        target_dir: Option<PathBuf>,
        /// Template's version, using main branch if not specified
        #[clap(short, long)]
        version: Option<String>,
    },
}

#[derive(new)]
pub struct CWModule {}

impl<'a> CWModule {
    fn new_<Ctx: Context<'a, CWConfig>>(
        ctx: Ctx,
        name: &str,
        version: Option<String>,
        target_dir: Option<PathBuf>,
    ) -> Result<()> {
        let cfg = ctx.config()?;
        let repo = &cfg.template_repo;
        let version = version.unwrap_or("main".to_string());
        let target_dir =
            target_dir.unwrap_or(ctx.root()?.join(PathBuf::from(cfg.contract_dir.as_str())));

        let cw_template =
            Template::new(name.to_string(), repo.to_owned(), version, target_dir, None);
        cw_template.generate()
    }
}

impl<'a> Module<'a, CWConfig, CWCmd, anyhow::Error> for CWModule {
    fn execute<Ctx: Context<'a, CWConfig>>(ctx: Ctx, cmd: &CWCmd) -> Result<(), anyhow::Error> {
        match cmd {
            CWCmd::New {
                name,
                target_dir,
                version,
            } => CWModule::new_(ctx, name, version.to_owned(), target_dir.to_owned()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::{prelude::*, TempDir};
    use cargo_toml::{Dependency, DependencyDetail, Manifest};
    use predicates::prelude::*;
    use serial_test::serial;
    use std::{env, fs, path::Path};

    struct CWContext {}
    impl<'a> Context<'a, CWConfig> for CWContext {}

    #[test]
    #[serial]
    fn generate_contract_with_default_version_and_path() {
        let temp = setup();

        temp.child("contracts").assert(predicate::path::missing());
        temp.child("contracts/counter-1")
            .assert(predicate::path::missing());
        temp.child("contracts/counter-2")
            .assert(predicate::path::missing());

        CWModule::execute(
            CWContext {},
            &CWCmd::New {
                name: "counter-1".to_string(),
                target_dir: None,
                version: None,
            },
        )
        .unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());

        // cd into contract before running command
        env::set_current_dir(temp.to_path_buf().join(PathBuf::from("contracts"))).unwrap();

        CWModule::execute(
            CWContext {},
            &CWCmd::New {
                name: "counter-2".to_string(),
                target_dir: None,
                version: None,
            },
        )
        .unwrap();
        temp.child("contracts/counter-2")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }

    #[test]
    #[serial]
    fn generate_contract_with_default_version_and_path_from_child_dir() {
        let temp = setup();

        temp.child("contracts").assert(predicate::path::missing());
        temp.child("contracts/counter-1")
            .assert(predicate::path::missing());
        temp.child("contracts/counter-2")
            .assert(predicate::path::missing());

        CWModule::execute(
            CWContext {},
            &CWCmd::New {
                name: "counter-1".to_string(),
                target_dir: None,
                version: None,
            },
        )
        .unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());

        CWModule::execute(
            CWContext {},
            &CWCmd::New {
                name: "counter-2".to_string(),
                target_dir: None,
                version: None,
            },
        )
        .unwrap();
        temp.child("contracts/counter-2")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }

    #[test]
    #[serial]
    fn generate_contract_with_custom_version() {
        let temp = setup();

        temp.child("contracts").assert(predicate::path::missing());
        temp.child("contracts/counter-1")
            .assert(predicate::path::missing());
        temp.child("contracts/counter-2")
            .assert(predicate::path::missing());

        CWModule::execute(
            CWContext {},
            &CWCmd::New {
                name: "counter-1".to_string(),
                target_dir: None,
                version: Some("0.16".into()),
            },
        )
        .unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());
        assert_version(Path::new("contracts/counter-1/Cargo.toml"), "0.16");

        CWModule::execute(
            CWContext {},
            &CWCmd::New {
                name: "counter-2".to_string(),
                target_dir: None,
                version: Some("0.16".into()),
            },
        )
        .unwrap();
        temp.child("contracts/counter-2")
            .assert(predicate::path::exists());
        assert_version(Path::new("contracts/counter-2/Cargo.toml"), "0.16");

        temp.close().unwrap();
    }

    #[test]
    #[serial]
    fn generate_contract_with_custom_path() {
        let temp = setup();
        env::set_current_dir(&temp).unwrap();

        temp.child("custom-path").assert(predicate::path::missing());
        temp.child("custom-path/counter-1")
            .assert(predicate::path::missing());
        temp.child("custom-path/counter-2")
            .assert(predicate::path::missing());

        CWModule::execute(
            CWContext {},
            &CWCmd::New {
                name: "counter-1".to_string(),
                target_dir: Some("custom-path".into()),
                version: None,
            },
        )
        .unwrap();
        temp.child("custom-path/counter-1")
            .assert(predicate::path::exists());

        CWModule::execute(
            CWContext {},
            &CWCmd::New {
                name: "counter-2".to_string(),
                target_dir: Some("custom-path".into()),
                version: None,
            },
        )
        .unwrap();
        temp.child("custom-path/counter-2")
            .assert(predicate::path::exists());

        temp.close().unwrap();
    }

    fn setup() -> TempDir {
        let temp = assert_fs::TempDir::new().unwrap();
        env::set_current_dir(&temp).unwrap();
        fs::File::create("Protostar.toml").unwrap();
        temp
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
