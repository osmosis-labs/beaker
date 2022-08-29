use std::str::FromStr;

use crate::framework::config::Network;
use anyhow::{anyhow, Ok};
use anyhow::{Context, Result};
use cosmos_sdk_proto::cosmos::gov::v1beta1::Proposal;
use cosmrs::abci::GasInfo;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::proto::cosmos::auth::v1beta1::BaseAccount;
use cosmrs::tendermint::abci::tag::{Key, Value};

use cosmrs::tx::{self, SignDoc, SignerInfo};
use cosmrs::{dev, AccountId, Coin};
use cosmrs::{rpc, tx::Fee, Any};
use prost::Message;

use super::gas::Gas;

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

    #[allow(deprecated)]
    pub async fn simulate(&self, tx_bytes: Vec<u8>) -> Result<GasInfo> {
        use cosmos_sdk_proto::cosmos::tx::v1beta1::*;
        let grpc_endpoint = self.network.grpc_endpoint();

        let mut c = service_client::ServiceClient::connect(self.network.grpc_endpoint().clone())
            .await
            .context(format!("Unable to connect to {grpc_endpoint}"))?;

        let res = c
            .simulate(SimulateRequest { tx: None, tx_bytes })
            .await?
            .into_inner()
            .gas_info;

        let gas_info = res.with_context(|| "Unable to extract gas info")?;

        gas_info
            .try_into()
            .map_err(|e: cosmrs::ErrorReport| anyhow!(e))
    }

    pub async fn query_smart(&self, address: String, query_data: Vec<u8>) -> Result<Vec<u8>> {
        use cosmos_sdk_proto::cosmwasm::wasm::v1::*;
        let grpc_endpoint = self.network.grpc_endpoint();

        let mut c = query_client::QueryClient::connect(self.network.grpc_endpoint().clone())
            .await
            .context(format!("Unable to connect to {grpc_endpoint}"))?;

        let res = c
            .smart_contract_state(QuerySmartContractStateRequest {
                address,
                query_data,
            })
            .await?
            .into_inner()
            .data;

        Ok(res)
    }

    pub async fn proposal(&self, proposal_id: &u64) -> Result<Proposal> {
        use cosmos_sdk_proto::cosmos::gov::v1beta1::*;
        let grpc_endpoint = self.network.grpc_endpoint();

        let mut c = query_client::QueryClient::connect(self.network.grpc_endpoint().clone())
            .await
            .context(format!("Unable to connect to {grpc_endpoint}"))?;

        let res = c
            .proposal(QueryProposalRequest {
                proposal_id: *proposal_id,
            })
            .await?
            .into_inner()
            .proposal;

        res.with_context(|| format!("Unable to find proposal with id {proposal_id}"))
    }

    async fn gov_params(
        &self,
        params_type: &str,
    ) -> Result<cosmos_sdk_proto::cosmos::gov::v1beta1::QueryParamsResponse> {
        use cosmos_sdk_proto::cosmos::gov::v1beta1::*;
        let grpc_endpoint = self.network.grpc_endpoint();

        let mut c = query_client::QueryClient::connect(self.network.grpc_endpoint().clone())
            .await
            .context(format!("Unable to connect to {grpc_endpoint}"))?;

        let res = c
            .params(QueryParamsRequest {
                params_type: params_type.to_string(), // voting, tallying, deposit
            })
            .await?
            .into_inner();

        Ok(res)
    }
    pub async fn gov_params_deposit(
        &self,
    ) -> Result<cosmos_sdk_proto::cosmos::gov::v1beta1::DepositParams> {
        self.gov_params("deposit")
            .await?
            .deposit_params
            .with_context(|| "Deposit params is not available")
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

    pub async fn estimate_fee(
        &self,
        gas: Gas,
        account: &BaseAccount,
        tx_body: tx::Body,
    ) -> Result<Fee> {
        match gas {
            Gas::Specified(fee) => Ok(fee),
            Gas::Auto {
                gas_price,
                gas_adjustment,
            } => {
                let signer_info = SignerInfo::single_direct(
                    Some(self.signing_key.public_key()),
                    account.sequence,
                );
                let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(
                    Coin {
                        denom: gas_price.denom.clone(),
                        amount: 0u8.into(),
                    },
                    0u64,
                ));
                let sign_doc = SignDoc::new(
                    &tx_body,
                    &auth_info,
                    &self.inner.network.chain_id().parse().unwrap(),
                    account.account_number,
                )
                .unwrap();
                let tx_raw = sign_doc.sign(&self.signing_key).unwrap();
                let gas_info = self.inner.simulate(tx_raw.to_bytes().unwrap()).await?;
                let gas_limit: u64 = gas_info.gas_used.into();
                let gas_limit = ((gas_limit as f64) * (gas_adjustment as f64)).ceil();

                let amount = Coin {
                    denom: gas_price.denom,
                    amount: (((gas_limit as f64) * (gas_price.amount as f64)).ceil() as u64).into(),
                };

                Ok(Fee::from_amount_and_gas(amount, gas_limit as u64))
            }
        }
    }

    pub async fn sign_and_broadcast(
        &self,
        msgs: Vec<Any>,
        gas: &Gas,
        memo: &str,
        timeout_height: &u32,
    ) -> Result<TxCommitResponse> {
        let acc = self
            .inner
            .account(self.signer_account_id().as_ref())
            .await
            .with_context(|| "Account can't be initialized")?;

        let tx_body = tx::Body::new(msgs, memo, *timeout_height);

        let fee = self
            .estimate_fee(gas.clone(), &acc, tx_body.clone())
            .await?;

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

        let rpc_client = rpc::HttpClient::new(self.inner.network.rpc_endpoint().as_str()).unwrap();
        dev::poll_for_first_block(&rpc_client).await;

        let tx_commit_response = tx_raw.broadcast_commit(&rpc_client).await.unwrap();

        if tx_commit_response.check_tx.code.is_err() {
            return Err(anyhow!(
                "check_tx failed: {:?}",
                tx_commit_response.check_tx
            ));
        }

        if tx_commit_response.deliver_tx.code.is_err() {
            return Err(anyhow!(
                "deliver_tx failed: {:?}",
                tx_commit_response.deliver_tx
            ));
        }

        // dev::poll_for_tx(&rpc_client, tx_commit_response.hash).await;

        Ok(tx_commit_response)
    }
}
