use anyhow::{bail, Context, Result};
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::{bip32, crypto::secp256k1};
use prost::Message;

use crate::framework::config::Account;
pub struct Client {
    base_url: String,
    chain_id: String,
    address_path: String,
    grpc_port: String,
    rpc_port: String,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            chain_id: Default::default(),
            address_path: Default::default(),
            base_url: Default::default(),
            grpc_port: "9090".to_string(),
            rpc_port: "26657".to_string(),
        }
    }
}

impl Client {
    pub fn _new(
        base_url: &str,
        chain_id: &str,
        address_path: &str,
        grpc_port: &str,
        rpc_port: &str,
    ) -> Self {
        Client {
            base_url: base_url.to_string(),
            chain_id: chain_id.to_string(),
            address_path: address_path.to_string(),
            grpc_port: grpc_port.to_string(),
            rpc_port: rpc_port.to_string(),
        }
    }

    pub fn local(chain_id: &str, address_path: &str) -> Self {
        Client {
            base_url: "http://localhost".to_string(),
            chain_id: chain_id.to_string(),
            address_path: address_path.to_string(),
            ..Client::default()
        }
    }

    pub fn grpc_address(&self) -> String {
        let base_url = self.base_url.as_str();
        let grpc_port = self.grpc_port.as_str();
        format!("{base_url}:{grpc_port}")
    }
    pub fn rpc_address(&self) -> String {
        let base_url = self.base_url.as_str();
        let rpc_port = self.rpc_port.as_str();
        format!("{base_url}:{rpc_port}")
    }

    pub async fn account(&self, address: &str) -> Result<BaseAccount> {
        use cosmos_sdk_proto::cosmos::auth::v1beta1::*;
        let grpc_address = self.grpc_address();

        let mut c = query_client::QueryClient::connect(self.grpc_address())
            .await
            .context(format!("Unable to connect to {grpc_address}"))?;

        let res = c
            .account(QueryAccountRequest {
                address: address.into(),
            })
            .await?
            .into_inner()
            .account
            .context("Account not found")?;

        BaseAccount::decode(res.value.as_slice()).context("Unable to decode BaseAccount")
    }
}

pub fn extract_private_key(
    global_config: &crate::framework::config::GlobalConfig,
    signer_account: Option<&str>,
    signer_mnemonic: Option<&str>,
    signer_private_key: Option<&str>,
) -> Result<secp256k1::SigningKey, anyhow::Error> {
    let derivation_path = global_config.derivation_path();
    let signer_priv = if let Some(signer_account) = signer_account {
        match global_config.accounts().get(signer_account) {
            None => bail!("signer account: `{signer_account}` is not defined"),
            Some(Account::FromMnemonic { mnemonic }) => {
                from_mnemonic(mnemonic.as_str(), derivation_path)
            }
            Some(Account::FromPrivateKey { private_key }) => {
                Ok(secp256k1::SigningKey::from_bytes(&base64::decode(private_key)?).unwrap())
            }
        }
    } else if let Some(signer_mnemonic) = signer_mnemonic {
        from_mnemonic(signer_mnemonic, derivation_path)
    } else if let Some(signer_private_key) = signer_private_key {
        Ok(secp256k1::SigningKey::from_bytes(&base64::decode(signer_private_key)?).unwrap())
    } else {
        bail!("Unable to retrive signer private key")
    }?;
    Ok(signer_priv)
}

fn from_mnemonic(
    phrase: &str,
    derivation_path: &str,
) -> Result<secp256k1::SigningKey, anyhow::Error> {
    let seed = bip32::Mnemonic::new(phrase, bip32::Language::English)?.to_seed("");
    let xprv = bip32::XPrv::derive_from_path(seed, &derivation_path.parse()?)?;
    let signer_priv: secp256k1::SigningKey = xprv.into();
    Ok(signer_priv)
}
