use crate::attrs_format;
use crate::modules::wasm::config::WasmConfig;
use crate::support::cosmos::ResponseValuePicker;
use crate::support::future::block;
use crate::support::gas::Gas;
use crate::support::hooks::use_code_id;
use crate::support::ops_response::OpResponseDisplay;
use crate::support::state::State;
use crate::{framework::Context, support::cosmos::Client};

use anyhow::Context as _;
use anyhow::Result;
use cosmos_sdk_proto::cosmwasm::wasm::v1::MsgMigrateContract;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::MessageExt;
use std::fs;

#[allow(clippy::too_many_arguments)]
pub fn migrate<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    label: &str,
    raw: Option<&String>,
    no_proposal_sync: bool,
    yes: bool,
    network: &str,
    timeout_height: &u32,
    gas: &Gas,
    signing_key: SigningKey,
) -> Result<MigrateResponse> {
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
        state.clone(),
        contract_name,
        no_proposal_sync,
        yes,
    )?;

    let wasm_ref = state.get_ref(network, contract_name)?;
    let contract = wasm_ref
        .addresses()
        .get(label)
        .with_context(|| format!("Unable to retrieve contract for {contract_name}:{label}"))?;

    let msg_migrate_contract = MsgMigrateContract {
        sender: client.signer_account_id().to_string(),
        contract: contract.to_string(),
        code_id,
        msg: raw
            .map(|s| s.as_bytes().to_vec())
            .map(Ok)
            .unwrap_or_else(|| {
                let path = ctx
                    .root()?
                    .join("contracts")
                    .join(contract_name)
                    .join("migrate-msgs")
                    .join(format!("{label}.json"));
                fs::read_to_string(&path)
                    .with_context(|| format!("Unable to migrate with `{}`", path.to_string_lossy()))
                    .map(|s| s.as_bytes().to_vec())
            })?,
    };

    block(async {
        let response = client
            .sign_and_broadcast(
                vec![msg_migrate_contract.to_any().unwrap()],
                gas,
                "",
                timeout_height,
            )
            .await?;

        let contract_address = response.pick("migrate", "_contract_address").to_string();
        let code_id = response.pick("migrate", "code_id").to_string();

        let migrate_response = MigrateResponse {
            code_id: code_id.parse()?,
            contract_address: contract_address.clone(),
            label: label.to_string(),
            creator: msg_migrate_contract.sender.to_string(),
        };

        migrate_response.log();

        State::update_state_file(
            network_info.network_variant(),
            ctx.root()?,
            &|s: &State| -> State {
                s.update_address(network, contract_name, label, &contract_address)
            },
        )?;

        Ok(migrate_response)
    })
}

#[allow(dead_code)]
pub struct MigrateResponse {
    pub label: String,
    pub contract_address: String,
    pub code_id: u64,
    pub creator: String,
}

impl OpResponseDisplay for MigrateResponse {
    fn headline() -> &'static str {
        "Contract migrated successfully!! 🎉 "
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | label, contract_address, code_id, creator }
    }
}
