use super::config::KeyConfig;
use crate::framework::{Context, Module};
use crate::support::signer::SigningKeyExt;
use anyhow::{Context as _, Ok, Result};
use clap::Subcommand;
use cosmrs::bip32;
use cosmrs::bip32::secp256k1::elliptic_curve::rand_core::OsRng;
use cosmrs::crypto::secp256k1::SigningKey;

// TODO:
// -- DONE
// beaker key set username "camel hours ..."
// beaker key address <username>
// beaker key gen username [--show]
// beaker key delete username
//
// -- TODO
// confirm on overrride
// prevent wrong mnemonic to not pass
// add alias

#[derive(Subcommand, Debug)]
pub enum KeyCmd {
    /// Create new key or update existing key
    Set {
        /// Name of the key to create or update
        name: String,
        /// Mnemonic string to store as an entry
        mnemonic: String,
    },
    /// Delete existing key
    #[clap(alias = "del")]
    Delete {
        /// Name of the key to create or update
        name: String,
    },
    /// Get address from keyring's stored key
    #[clap(alias = "addr")]
    Address {
        /// Name of the key to create or update
        name: String,
    },
    /// Generate new mnemonic
    #[clap(alias = "gen")]
    Generate {
        /// Name of the key to create or update
        name: String,

        /// Show mnemonic in the console if set, keep it secret otherwise
        #[clap(long)]
        show: bool,
    },
}

pub struct KeyModule {}

impl<'a> Module<'a, KeyConfig, KeyCmd, anyhow::Error> for KeyModule {
    fn execute<Ctx: Context<'a, KeyConfig>>(ctx: Ctx, cmd: &KeyCmd) -> Result<(), anyhow::Error> {
        match cmd {
            KeyCmd::Set { name, mnemonic } => {
                let entry = keyring::Entry::new(&ctx.config()?.service, name);
                entry
                    .set_password(mnemonic)
                    .with_context(|| "Unable to set key")
            }
            KeyCmd::Delete { name } => {
                let entry = keyring::Entry::new(&ctx.config()?.service, name);
                entry
                    .delete_password()
                    .with_context(|| "Unable to delete key")
            }
            KeyCmd::Address { name } => {
                let entry = keyring::Entry::new(&ctx.config()?.service, name);
                let global_config = ctx.global_config()?;

                let mnemonic = entry.get_password()?;
                let derivation_path = global_config.derivation_path();
                let address = SigningKey::from_mnemonic(&mnemonic, derivation_path)?
                    .public_key()
                    .account_id(global_config.account_prefix())
                    .unwrap()
                    .to_string();

                println!("{}", address);
                Ok(())
            }
            KeyCmd::Generate { name, show } => {
                let mnemonic = bip32::Mnemonic::random(OsRng, bip32::Language::English);
                let mnemonic = mnemonic.phrase();

                let entry = keyring::Entry::new(&ctx.config()?.service, name);

                if *show {
                    println!("{}", mnemonic);
                }

                entry
                    .set_password(mnemonic)
                    .with_context(|| "Unable to set key")
            }
        }
    }
}
