use std::str::FromStr;

use crate::framework::config::Network;
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

#[derive(Clone, Debug)]
pub struct Client {
    network: Network,
}

impl Client {
    pub fn new(network: Network) -> Self {
        Client { network }
    }

    pub fn to_signing_client(
        &self,
        signing_key: SigningKey,
        account_prefix: &str,
    ) -> SigningClient {
        SigningClient {
            inner: self.clone(),
            signing_key,
            account_prefix: account_prefix.to_string(),
        }
    }

    pub async fn account(&self, address: &str) -> Result<BaseAccount> {
        use cosmos_sdk_proto::cosmos::auth::v1beta1::*;
        let grpc_endpoint = self.network.grpc_endpoint();

        let mut c = query_client::QueryClient::connect(self.network.grpc_endpoint().clone())
            .await
            .context(format!("Unable to connect to {grpc_endpoint}"))?;

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
            &self.inner.network.chain_id().parse().unwrap(),
            acc.account_number,
        )
        .unwrap();
        let tx_raw = sign_doc.sign(&self.signing_key).unwrap();

        // === Poll for first block (Invariant)
        let rpc_client = rpc::HttpClient::new(self.inner.network.rpc_endpoint().as_str()).unwrap();
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
