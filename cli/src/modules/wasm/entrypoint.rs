use super::proposal::entrypoint::ProposalCmd;
use super::{args::BaseTxArgs, config::WasmConfig};
use super::{ops, proposal};
use crate::framework::{Context, Module};
use anyhow::Result;
use clap::Subcommand;
use cosmrs::tx::Fee;
use derive_new::new;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum WasmCmd {
    /// Create new CosmWasm contract from boilerplate
    New {
        /// Contract name
        contract_name: String,
        /// Path to store generated contract
        #[clap(short, long)]
        target_dir: Option<PathBuf>,
        /// Template's version, using main branch if not specified
        #[clap(short, long)]
        version: Option<String>,
    },
    /// Build .wasm for storing contract code on the blockchain
    Build {
        /// If set, the contract(s) will be optimized after build
        #[clap(short, long)]
        optimize: bool,
        /// Option for m1 user for wasm optimization, FOR TESTING ONLY, PRODUCTION BUILD SHOULD USE INTEL BUILD
        #[clap(short, long)]
        aarch64: bool,
    },
    /// Store .wasm on chain for later initialization
    StoreCode {
        /// Name of the contract to store
        contract_name: String,

        // TODO: implement --all flag
        #[clap(flatten)]
        base_tx_args: BaseTxArgs,
    },

    /// Instanitate .wasm stored on chain
    Instantiate {
        /// Name of the contract to instantiate
        contract_name: String,
        /// Label for the instantiated contract for later reference
        #[clap(short, long, default_value = "default")]
        label: String,
        /// Raw json string to use as instantiate msg
        #[clap(short, long)]
        raw: Option<String>,

        #[clap(flatten)]
        base_tx_args: BaseTxArgs,
    },
    /// Build, Optimize, Store code, and instantiate contract
    Deploy {
        /// Name of the contract to deploy
        contract_name: String,

        /// Label for the instantiated contract for later reference
        #[clap(short, long, default_value = "default")]
        label: String,

        /// Raw json string to use as instantiate msg
        #[clap(short, long)]
        raw: Option<String>,

        /// Use existing .wasm file to deploy if set to true
        #[clap(long)]
        no_rebuild: bool,

        #[clap(flatten)]
        base_tx_args: BaseTxArgs,
    },
    Proposal {
        #[clap(subcommand)]
        cmd: ProposalCmd,
    },
}

#[derive(new)]
pub struct WasmModule {}

impl<'a> Module<'a, WasmConfig, WasmCmd, anyhow::Error> for WasmModule {
    fn execute<Ctx: Context<'a, WasmConfig>>(ctx: Ctx, cmd: &WasmCmd) -> Result<(), anyhow::Error> {
        match cmd {
            WasmCmd::New {
                contract_name: name,
                target_dir, // TODO: Rremove this
                version,
            } => ops::new(&ctx, name, version.to_owned(), target_dir.to_owned()),
            WasmCmd::Build { optimize, aarch64 } => ops::build(&ctx, optimize, aarch64), // TODO: change optimize -> no-optimize
            WasmCmd::StoreCode {
                contract_name,
                base_tx_args,
            } => {
                let BaseTxArgs {
                    network,
                    signer_args,
                    gas_args,
                    timeout_height,
                }: &BaseTxArgs = base_tx_args;

                ops::store_code(
                    &ctx,
                    contract_name,
                    network,
                    &Fee::try_from(gas_args)?,
                    timeout_height,
                    signer_args.private_key(&ctx.global_config()?)?,
                )?;
                Ok(())
            }

            WasmCmd::Instantiate {
                contract_name,
                label,
                raw,
                base_tx_args,
            } => {
                let BaseTxArgs {
                    network,
                    signer_args,
                    gas_args,
                    timeout_height,
                }: &BaseTxArgs = base_tx_args;
                ops::instantiate(
                    &ctx,
                    contract_name,
                    label.as_str(),
                    raw.as_ref(),
                    network,
                    timeout_height,
                    &Fee::try_from(gas_args)?,
                    signer_args.private_key(&ctx.global_config()?)?,
                )?;
                Ok(())
            }
            WasmCmd::Deploy {
                contract_name,
                label,
                raw,
                no_rebuild,
                base_tx_args,
            } => {
                let BaseTxArgs {
                    network,
                    signer_args,
                    gas_args,
                    timeout_height,
                }: &BaseTxArgs = base_tx_args;
                ops::deploy(
                    &ctx,
                    contract_name,
                    label.as_str(),
                    raw.as_ref(),
                    network,
                    timeout_height,
                    &Fee::try_from(gas_args)?,
                    signer_args.private_key(&ctx.global_config()?)?,
                    signer_args.private_key(&ctx.global_config()?)?,
                    no_rebuild,
                )?;
                Ok(())
            }
            WasmCmd::Proposal { cmd } => proposal::entrypoint::execute(ctx, cmd),
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
    impl<'a> Context<'a, WasmConfig> for CWContext {}

    #[test]
    #[serial]
    fn generate_contract_with_default_version_and_path() {
        let temp = setup();

        temp.child("contracts").assert(predicate::path::missing());
        temp.child("contracts/counter-1")
            .assert(predicate::path::missing());
        temp.child("contracts/counter-2")
            .assert(predicate::path::missing());

        WasmModule::execute(
            CWContext {},
            &WasmCmd::New {
                contract_name: "counter-1".to_string(),
                version: None,
                target_dir: None,
            },
        )
        .unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());

        // cd into contract before running command
        env::set_current_dir(temp.to_path_buf().join(PathBuf::from("contracts"))).unwrap();

        WasmModule::execute(
            CWContext {},
            &WasmCmd::New {
                contract_name: "counter-2".to_string(),
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

        WasmModule::execute(
            CWContext {},
            &WasmCmd::New {
                contract_name: "counter-1".to_string(),
                target_dir: None,
                version: None,
            },
        )
        .unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());

        WasmModule::execute(
            CWContext {},
            &WasmCmd::New {
                contract_name: "counter-2".to_string(),
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

        WasmModule::execute(
            CWContext {},
            &WasmCmd::New {
                contract_name: "counter-1".to_string(),
                target_dir: None,
                version: Some("0.16".into()),
            },
        )
        .unwrap();
        temp.child("contracts/counter-1")
            .assert(predicate::path::exists());
        assert_version(Path::new("contracts/counter-1/Cargo.toml"), "0.16");

        WasmModule::execute(
            CWContext {},
            &WasmCmd::New {
                contract_name: "counter-2".to_string(),
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

        WasmModule::execute(
            CWContext {},
            &WasmCmd::New {
                contract_name: "counter-1".to_string(),
                target_dir: Some("custom-path".into()),
                version: None,
            },
        )
        .unwrap();
        temp.child("custom-path/counter-1")
            .assert(predicate::path::exists());

        WasmModule::execute(
            CWContext {},
            &WasmCmd::New {
                contract_name: "counter-2".to_string(),
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
        fs::File::create("Beaker.toml").unwrap();
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
