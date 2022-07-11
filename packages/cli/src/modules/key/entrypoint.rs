use super::config::KeyConfig;
use crate::framework::{Context, Module};
use crate::support::signer::SigningKeyExt;
use anyhow::{Context as _, Ok, Result};
use clap::Subcommand;
use cosmrs::bip32;
use cosmrs::bip32::secp256k1::elliptic_curve::rand_core::OsRng;
use cosmrs::crypto::secp256k1::SigningKey;
use dialoguer::Confirm;
use keyring::Entry;

#[derive(Subcommand, Debug)]
pub enum KeyCmd {
    /// Create new key or update existing key
    Set {
        /// Name of the key to create or update
        name: String,

        /// Mnemonic string to store as an entry
        mnemonic: String,

        /// Agree to all prompts
        #[clap(short, long)]
        yes: bool,
    },
    /// Delete existing key
    #[clap(alias = "del")]
    Delete {
        /// Name of the key to create or update
        name: String,

        /// Agree to all prompts
        #[clap(short, long)]
        yes: bool,
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

        /// Agree to all prompts
        #[clap(short, long)]
        yes: bool,
    },
}

pub struct KeyModule {}

impl<'a> Module<'a, KeyConfig, KeyCmd, anyhow::Error> for KeyModule {
    fn execute<Ctx: Context<'a, KeyConfig>>(ctx: Ctx, cmd: &KeyCmd) -> Result<(), anyhow::Error> {
        match cmd {
            KeyCmd::Set {
                name,
                mnemonic,
                yes,
            } => {
                let entry = keyring::Entry::new(&ctx.config()?.service, name);
                let global_config = ctx.global_config()?;
                let derivation_path = global_config.derivation_path();

                SigningKey::from_mnemonic(mnemonic, derivation_path)
                    .with_context(|| "Invalid phrase, if word length is not 24, please consider using 24-words mnemonic")?;

                confirm_override(&ctx.config()?.service, name, *yes)?;
                entry
                    .set_password(mnemonic)
                    .with_context(|| "Unable to set key")
            }
            KeyCmd::Delete { name, yes } => {
                let entry = keyring::Entry::new(&ctx.config()?.service, name);

                confirm_deletion(&ctx.config()?.service, name, *yes)?;
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
            KeyCmd::Generate { name, show, yes } => {
                let mnemonic = bip32::Mnemonic::random(OsRng, bip32::Language::English);
                let mnemonic = mnemonic.phrase();

                let entry = keyring::Entry::new(&ctx.config()?.service, name);

                confirm_override(&ctx.config()?.service, name, *yes)?;

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

fn confirm_override(service: &str, name: &str, yes: bool) -> Result<bool, std::io::Error> {
    let entry = Entry::new(service, name);
    let exists = entry.get_password().is_ok();

    if yes || !exists {
        return core::result::Result::Ok(true);
    }

    Confirm::new()
        .with_prompt(format!(
            " > Key with name `{}` already exists. Do you want to override?",
            name
        ))
        .interact()
}

fn confirm_deletion(service: &str, name: &str, yes: bool) -> Result<bool, std::io::Error> {
    let entry = Entry::new(service, name);
    let exists = entry.get_password().is_ok();

    if yes || !exists {
        return core::result::Result::Ok(true);
    }

    Confirm::new()
        .with_prompt(format!(" > Do you want to delete `{}`?", name))
        .interact()
}
