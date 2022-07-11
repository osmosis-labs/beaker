use anyhow::bail;
use clap::Parser;
use cosmrs::{bip32, crypto::secp256k1::SigningKey};
use keyring::Entry;

use crate::{framework::config::Account, modules::key::config::SERVICE};

// TODO:
// - [x] make this a group
// - [ ] add signer_keyring

const SIGNER_GROUP: &str = "signer";

#[derive(Debug, Parser, Clone)]
#[clap(group = clap::ArgGroup::new(SIGNER_GROUP).multiple(false))]
pub struct SignerArgs {
    /// Specifies predefined account as a tx signer
    #[clap(long, group = SIGNER_GROUP)]
    pub signer_account: Option<String>,

    /// Specifies private_key as a tx signer (base64 encoded string)
    #[clap(long, group = SIGNER_GROUP)]
    pub signer_keyring: Option<String>,

    /// Specifies mnemonic as a tx signer
    #[clap(long, group = SIGNER_GROUP)]
    pub signer_mnemonic: Option<String>,

    /// Specifies private_key as a tx signer (base64 encoded string)
    #[clap(long, group = SIGNER_GROUP)]
    pub signer_private_key: Option<String>,
}

impl SignerArgs {
    pub fn private_key(
        &self,
        global_config: &crate::framework::config::GlobalConfig,
    ) -> Result<SigningKey, anyhow::Error> {
        let Self {
            signer_account,
            signer_keyring,
            signer_mnemonic,
            signer_private_key,
        } = self;
        let derivation_path = global_config.derivation_path();
        let signer_priv = if let Some(signer_account) = signer_account {
            match global_config.accounts().get(signer_account) {
                None => bail!("signer account: `{signer_account}` is not defined"),
                Some(Account::FromMnemonic { mnemonic }) => {
                    SigningKey::from_mnemonic(mnemonic.as_str(), derivation_path)
                }
                Some(Account::FromPrivateKey { private_key }) => {
                    Ok(SigningKey::from_bytes(&base64::decode(private_key)?).unwrap())
                }
            }
        } else if let Some(signer_keyring) = signer_keyring {
            let mnemonic = Entry::new(SERVICE, signer_keyring).get_password()?;
            SigningKey::from_mnemonic(&mnemonic, derivation_path)
        } else if let Some(signer_mnemonic) = signer_mnemonic {
            SigningKey::from_mnemonic(signer_mnemonic, derivation_path)
        } else if let Some(signer_private_key) = signer_private_key {
            Ok(SigningKey::from_bytes(&base64::decode(signer_private_key)?).unwrap())
        } else {
            bail!("Unable to retrive signer private key")
        }?;
        Ok(signer_priv)
    }
}

pub trait SigningKeyExt {
    fn from_mnemonic(phrase: &str, derivation_path: &str) -> Result<SigningKey, anyhow::Error> {
        let seed = bip32::Mnemonic::new(phrase, bip32::Language::English)?.to_seed("");
        let xprv = bip32::XPrv::derive_from_path(seed, &derivation_path.parse()?)?;
        let signer_priv: SigningKey = xprv.into();
        Ok(signer_priv)
    }
}

impl SigningKeyExt for SigningKey {}
