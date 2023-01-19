use crate::attrs_format;
use crate::modules::wasm::config::WasmConfig;
use crate::support::coin::Coins;
use crate::support::cosmos::ResponseValuePicker;
use crate::support::future::block;
use crate::support::gas::Gas;
use crate::support::ops_response::OpResponseDisplay;
use crate::support::state::State;
use crate::{framework::Context, support::cosmos::Client};
use anyhow::anyhow;
use anyhow::Context as _;
use anyhow::Result;
use cosmrs::cosmwasm::MsgExecuteContract;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::Msg;
use cosmrs::AccountId;

use std::{fs, vec};

#[allow(clippy::too_many_arguments)]
pub fn execute<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    label: &str,
    raw: Option<&String>,
    funds: Coins,
    network: &str,
    timeout_height: &u32,
    gas: &Gas,
    signing_key: SigningKey,
) -> Result<ExecuteResponse> {
    let global_config = ctx.global_config()?;
    let account_prefix = global_config.account_prefix().as_str();

    let network_info = global_config
        .networks()
        .get(network)
        .with_context(|| format!("Unable to find network config: {network}"))?
        .to_owned();

    let client = Client::new(network_info.clone()).to_signing_client(signing_key, account_prefix);
    let state = State::load_by_network(network_info, ctx.root()?)?;

    let contract = state
        .get_ref(network, contract_name)?
        .addresses()
        .get(label)
        .with_context(|| format!("Unable to retrieve contract for {contract_name}:{label}"))?
        .parse::<AccountId>()
        .map_err(|e| anyhow!(e))?;

    let msg_instantiate_contract = MsgExecuteContract {
        sender: client.signer_account_id(),
        contract,
        msg: raw
            .map(|s| s.as_bytes().to_vec())
            .map(Ok)
            .unwrap_or_else(|| {
                let path = ctx
                    .root()?
                    .join("contracts")
                    .join(contract_name)
                    .join("execute-msgs")
                    .join(format!("{label}.json"));
                fs::read_to_string(&path)
                    .with_context(|| format!("Unable to execute with `{}`", path.to_string_lossy()))
                    .map(|s| s.as_bytes().to_vec())
            })?,
        funds: funds.into(),
    };

    block(async {
        let response = client
            .sign_and_broadcast(
                vec![msg_instantiate_contract.to_any().unwrap()],
                gas,
                "",
                timeout_height,
            )
            .await?;

        let contract_address = response.pick("execute", "_contract_address");

        let execute_response = ExecuteResponse {
            contract_address,
            label: label.to_string(),
        };

        execute_response.log();

        Ok(execute_response)
    })
}

#[allow(dead_code)]
pub struct ExecuteResponse {
    pub label: String,
    pub contract_address: String,
}

impl OpResponseDisplay for ExecuteResponse {
    fn headline() -> &'static str {
        "Contract executed successfully!! 🎉 "
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | label, contract_address }
    }
}
