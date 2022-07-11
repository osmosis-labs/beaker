use crate::attrs_format;
use crate::modules::wasm::config::WasmConfig;
use crate::support::admin::compute_admin;
use crate::support::coin::Coins;
use crate::support::cosmos::ResponseValuePicker;
use crate::support::future::block;
use crate::support::gas::Gas;
use crate::support::hooks::use_code_id;
use crate::support::ops_response::OpResponseDisplay;
use crate::support::state::State;
use crate::{framework::Context, support::cosmos::Client};
use anyhow::Context as _;
use anyhow::Result;
use cosmrs::cosmwasm::MsgInstantiateContract;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::Msg;

use std::{fs, vec};

#[allow(clippy::too_many_arguments)]
pub fn instantiate<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    label: &str,
    raw: Option<&String>,
    admin: Option<&String>,
    no_proposal_sync: bool,
    yes: bool,
    funds: Coins,
    network: &str,
    timeout_height: &u32,
    gas: &Gas,
    signing_key: SigningKey,
) -> Result<InstantiateResponse> {
    let global_config = ctx.global_config()?;
    let account_prefix = global_config.account_prefix().as_str();

    let network_info = global_config
        .networks()
        .get(network)
        .with_context(|| format!("Unable to find network config: {network}"))?
        .to_owned();

    let client = Client::new(network_info.clone()).to_signing_client(signing_key, account_prefix);

    let state = State::load_by_network(network_info.clone(), ctx.root()?)?;

    let code_id = use_code_id(
        ctx,
        network,
        &network_info,
        state,
        contract_name,
        no_proposal_sync,
        yes,
    )?;

    let msg_instantiate_contract = MsgInstantiateContract {
        sender: client.signer_account_id(),
        admin: compute_admin(admin, client.signer_account_id())?,
        code_id,
        label: Some(label.to_string()),
        msg: raw
            .map(|s| s.as_bytes().to_vec())
            .map(Ok)
            .unwrap_or_else(|| {
                let path = ctx
                    .root()?
                    .join("contracts")
                    .join(contract_name)
                    .join("instantiate-msgs")
                    .join(format!("{label}.json"));
                fs::read_to_string(&path)
                    .with_context(|| {
                        format!("Unable to instantiate with `{}`", path.to_string_lossy())
                    })
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

        let contract_address = response
            .pick("instantiate", "_contract_address")
            .to_string();

        let instantiate_response = InstantiateResponse {
            code_id,
            contract_address: contract_address.clone(),
            label: msg_instantiate_contract
                .label
                .unwrap_or_else(|| "-".to_string()),
            creator: msg_instantiate_contract.sender.to_string(),
            admin: msg_instantiate_contract
                .admin
                .map(|a| a.to_string())
                .unwrap_or_else(|| "-".to_string()),
        };

        instantiate_response.log();

        State::update_state_file(
            network_info.network_variant(),
            ctx.root()?,
            &|s: &State| -> State {
                s.update_address(network, contract_name, label, &contract_address)
            },
        )?;

        Ok(instantiate_response)
    })
}

#[allow(dead_code)]
pub struct InstantiateResponse {
    pub label: String,
    pub contract_address: String,
    pub code_id: u64,
    pub creator: String,
    pub admin: String,
}

impl OpResponseDisplay for InstantiateResponse {
    fn headline() -> &'static str {
        "Contract instantiated successfully!! 🎉 "
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | label, contract_address, code_id, creator, admin }
    }
}
