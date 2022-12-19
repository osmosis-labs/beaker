use crate::attrs_format;
use crate::modules::wasm::WasmConfig;
use crate::support::future::block;
use crate::support::gas::Gas;
use crate::support::ops_response::OpResponseDisplay;

use anyhow::Context as _;

use cosmos_sdk_proto::cosmwasm::wasm::v1::MsgClearAdmin;
use cosmrs::tx::MessageExt;

use crate::support::state::State;

use crate::{framework::Context, support::cosmos::Client};

use anyhow::Result;

use cosmrs::crypto::secp256k1::SigningKey;

#[allow(clippy::too_many_arguments)]
pub fn clear_admin<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    label: &str,
    network: &str,
    gas: &Gas,
    timeout_height: &u32,
    signing_key: SigningKey,
) -> Result<ClearAdminResponse> {
    let global_config = ctx.global_config()?;
    let account_prefix = global_config.account_prefix().as_str();

    let network_info = global_config
        .networks()
        .get(network)
        .with_context(|| format!("Unable to find network config: {network}"))?
        .to_owned();

    let client = Client::new(network_info.clone()).to_signing_client(signing_key, account_prefix);

    let state = State::load_by_network(network_info, ctx.root()?)?;
    let wasm_ref = state.get_ref(network, contract_name)?;
    let contract = wasm_ref
        .addresses()
        .get(label)
        .with_context(|| format!("Unable to retrieve contract for {contract_name}:{label}"))?;

    let msg_clear_admin = MsgClearAdmin {
        sender: client.signer_account_id().to_string(),
        contract: contract.to_string(),
    }
    .to_any()
    .unwrap();

    block(async {
        let _response = client
            .sign_and_broadcast(vec![msg_clear_admin], gas, "", timeout_height)
            .await?;

        let clear_admin_response = ClearAdminResponse {
            contract: contract.to_string(),
            admin: "–".to_string(),
        };

        clear_admin_response.log();

        Ok(clear_admin_response)
    })
}

#[allow(dead_code)]
pub struct ClearAdminResponse {
    pub contract: String,
    pub admin: String,
}

impl OpResponseDisplay for ClearAdminResponse {
    fn headline() -> &'static str {
        "Clear admin successfully!! 🎉"
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | contract, admin }
    }
}
