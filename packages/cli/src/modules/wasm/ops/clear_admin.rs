use crate::attrs_format;
use crate::modules::wasm::WasmConfig;
use crate::support::future::block;
use crate::support::gas::Gas;
use crate::support::ops_response::OpResponseDisplay;
use anyhow::anyhow;
use anyhow::Context as _;
use cosmrs::cosmwasm::MsgClearAdmin;
use cosmrs::AccountId;

use crate::support::state::State;

use crate::{framework::Context, support::cosmos::Client};

use anyhow::Result;

use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::Msg;

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
    let contract = state
        .get_ref(network, contract_name)?
        .addresses()
        .get(label)
        .with_context(|| format!("Unable to retrieve contract for {contract_name}:{label}"))?
        .parse::<AccountId>()
        .map_err(|e| anyhow!(e))?;

    let msg_clear_admin = MsgClearAdmin {
        sender: client.signer_account_id(),
        contract: contract.clone(),
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
