use std::str::FromStr;

use anyhow::anyhow;
use anyhow::{Context, Result};
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tendermint::abci::tag::{Key, Value};
use cosmrs::tx::{self, SignDoc, SignerInfo};
use cosmrs::{dev, AccountId};
use cosmrs::{rpc, tx::Fee, Any};
use prost::Message;

pub type TxCommitResponse = rpc::endpoint::broadcast::tx_commit::Response;

pub trait ResponseValuePicker {
    fn pick(&self, event: &str, attribute: &str) -> Value;
}

impl ResponseValuePicker for TxCommitResponse {
    fn pick(&self, event: &str, attribute: &str) -> Value {
        self.deliver_tx
            .events
            .iter()
            .find(|e| e.type_str == event)
            .unwrap()
            .attributes
            .iter()
            .find(|a| a.key == Key::from_str(attribute).unwrap())
            .unwrap()
            .value
            .clone()
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
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

    pub fn to_signing_client(
        &self,
        signing_key: SigningKey,
        account_prefix: String,
    ) -> SigningClient {
        SigningClient {
            inner: self.clone(),
            signing_key,
            account_prefix,
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

pub struct SigningClient {
    inner: Client,
    account_prefix: String,
    signing_key: SigningKey,
}

impl SigningClient {
    pub fn signer_account_id(&self) -> AccountId {
        let signer_pub = self.signing_key.public_key();
        signer_pub.account_id(self.account_prefix.as_str()).unwrap()
    }

    pub async fn sign_and_broadcast(
        &self,
        msgs: Vec<Any>,
        fee: Fee,
        memo: &str,
        timeout_height: &u32,
    ) -> Result<TxCommitResponse> {
        let acc = self
            .inner
            .account(self.signer_account_id().as_ref())
            .await
            .with_context(|| "Account can't be initialized")?;

        // === Contract and Sign Tx (Invariant)
        let tx_body = tx::Body::new(msgs, memo, *timeout_height);
        let auth_info =
            SignerInfo::single_direct(Some(self.signing_key.public_key()), acc.sequence)
                .auth_info(fee.clone());
        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &self.inner.chain_id.parse().unwrap(),
            acc.account_number,
        )
        .unwrap();
        let tx_raw = sign_doc.sign(&self.signing_key).unwrap();

        // === Poll for first block (Invariant)
        let rpc_client = rpc::HttpClient::new(self.inner.rpc_address().as_str()).unwrap();
        dev::poll_for_first_block(&rpc_client).await;

        // === Broadcast (Invariant)
        let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();

        // === Check Tx (Invariant)
        if tx_commit_response.check_tx.code.is_err() {
            return Err(anyhow!(
                "check_tx failed: {:?}",
                tx_commit_response.check_tx
            ));
        }

        // === Deliver Tx (Invariant)
        if tx_commit_response.deliver_tx.code.is_err() {
            return Err(anyhow!(
                "deliver_tx failed: {:?}",
                tx_commit_response.deliver_tx
            ));
        }

        dev::poll_for_tx(&rpc_client, tx_commit_response.hash).await;

        Ok(tx_commit_response)
    }
}
