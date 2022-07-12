use crate::attrs_format;
use crate::modules::wasm::WasmConfig;
use crate::support::future::block;
use crate::support::gas::Gas;
use crate::support::ops_response::OpResponseDisplay;
use anyhow::anyhow;
use anyhow::Context as _;
use cosmrs::AccountId;

use crate::support::state::State;

use crate::{framework::Context, support::cosmos::Client};

use anyhow::Result;
use cosmrs::cosmwasm::MsgUpdateAdmin;

use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::Msg;

#[allow(clippy::too_many_arguments)]
pub fn update_admin<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    label: &str,
    network: &str,
    new_admin: &str,
    gas: &Gas,
    timeout_height: &u32,
    signing_key: SigningKey,
) -> Result<UpdateAdminResponse> {
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

    let msg_update_admin = MsgUpdateAdmin {
        sender: client.signer_account_id(),
        new_admin: new_admin
            .parse()
            .map_err(|e: cosmrs::ErrorReport| anyhow!(e))?,
        contract: contract.clone(),
    }
    .to_any()
    .unwrap();

    block(async {
        let _response = client
            .sign_and_broadcast(vec![msg_update_admin], gas, "", timeout_height)
            .await?;

        let update_admin_response = UpdateAdminResponse {
            new_admin: new_admin.to_string(),
            contract: contract.to_string(),
        };

        update_admin_response.log();

        Ok(update_admin_response)
    })
}

#[allow(dead_code)]
pub struct UpdateAdminResponse {
    pub contract: String,
    pub new_admin: String,
}

impl OpResponseDisplay for UpdateAdminResponse {
    fn headline() -> &'static str {
        "Update admin successfully!! 🎉"
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | contract, new_admin }
    }
}
