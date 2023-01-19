use crate::attrs_format;
use crate::modules::wasm::WasmConfig;
use crate::support::cosmos::ResponseValuePicker;
use crate::support::future::block;
use crate::support::gas::Gas;
use crate::support::ops_response::OpResponseDisplay;
use crate::support::permission::compute_instantiate_permission;
use crate::support::state::State;
use crate::support::wasm::read_wasm;
use crate::{framework::Context, support::cosmos::Client};
use anyhow::Context as _;
use anyhow::Result;
use cosmrs::cosmwasm::MsgStoreCode;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::Msg;

#[allow(clippy::too_many_arguments)]
pub fn store_code<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    network: &str,
    no_wasm_opt: &bool,
    permit_instantiate_only: &Option<String>,
    gas: &Gas,
    timeout_height: &u32,
    signing_key: SigningKey,
) -> Result<StoreCodeResponse> {
    let global_config = ctx.global_config()?;
    let account_prefix = global_config.account_prefix().as_str();

    let network_info = global_config
        .networks()
        .get(network)
        .with_context(|| format!("Unable to find network config: {network}"))?
        .to_owned();

    let client = Client::new(network_info.clone()).to_signing_client(signing_key, account_prefix);

    let instantiate_permission =
        compute_instantiate_permission(permit_instantiate_only, client.signer_account_id())?;

    let wasm = read_wasm(
        ctx.root()?,
        contract_name.replace('-', "_").as_str(), // Handles file name mismatch
        no_wasm_opt,
    )?;
    let msg_store_code = MsgStoreCode {
        sender: client.signer_account_id(),
        wasm_byte_code: wasm,
        instantiate_permission: instantiate_permission.clone(),
    }
    .to_any()
    .unwrap();

    block(async {
        let response = client
            .sign_and_broadcast(vec![msg_store_code], gas, "", timeout_height)
            .await?;

        let code_id: u64 = response.pick("store_code", "code_id").parse()?;
        let store_code_response = StoreCodeResponse {
            code_id,
            instantiate_permission: instantiate_permission
                .map(|p| format!("only_address | {}", p.address))
                .unwrap_or_else(|| "–".to_string()),
        };

        State::update_state_file(
            network_info.network_variant(),
            ctx.root()?,
            &|s: &State| -> State { s.update_code_id(network, contract_name, &code_id) },
        )?;
        store_code_response.log();

        Ok(store_code_response)
    })
}

#[allow(dead_code)]
pub struct StoreCodeResponse {
    pub code_id: u64,
    pub instantiate_permission: String,
}

impl OpResponseDisplay for StoreCodeResponse {
    fn headline() -> &'static str {
        "Code stored successfully!! 🎉"
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | code_id, instantiate_permission }
    }
}
