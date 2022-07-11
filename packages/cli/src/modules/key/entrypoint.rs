use super::config::KeyConfig;
use crate::framework::{Context, Module};
use crate::support::signer::SigningKeyExt;
use anyhow::{Context as _, Result};
use clap::Subcommand;
use cosmrs::crypto::secp256k1::SigningKey;

// TODO:
// -- DONE
// beaker key set username "camel hours ..."
// beaker key address <username>
//
// -- TODO
// beaker key list
// beaker key gen username
// beaker key delete username

#[derive(Subcommand, Debug)]
pub enum KeyCmd {
    /// Create new key or update existing key
    Set {
        /// Name of the key to create or update
        name: String,
        /// Mnemonic string to store as an entry
        mnemonic: String,
    },
    /// Get address from keyring's stored key
    Address {
        /// Name of the key to create or update
        name: String,
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
        }
    }
}
