use anyhow::{Context, Result};
use cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount;

use prost::Message;

#[allow(dead_code)]
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
