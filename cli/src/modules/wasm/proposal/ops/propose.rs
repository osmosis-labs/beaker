use crate::attrs_format;
use crate::support::coin::Coins;
use crate::support::cosmos::ResponseValuePicker;
use crate::support::future::block;
use crate::support::gas::Gas;
use crate::support::ops_response::OpResponseDisplay;
use crate::support::proto::MessageExt;
use crate::support::state::State;
use crate::support::wasm::read_wasm;
use crate::{framework::Context, modules::wasm::WasmConfig, support::cosmos::Client};
use anyhow::{Context as _, Result};
use cosmos_sdk_proto::cosmos::gov::v1beta1::MsgSubmitProposal;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::Any;
use std::vec;

#[allow(clippy::too_many_arguments)]
pub fn propose_store_code<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    title: &str,
    description: &str,
    deposit: Coins,
    network: &str,
    gas: &Gas,
    timeout_height: &u32,
    signing_key: SigningKey,
) -> Result<ProposeStoreCodeResponse> {
    let global_config = ctx.global_config()?;
    let account_prefix = global_config.account_prefix().as_str();
    let no_wasm_opt = &false;

    let network_info = global_config
        .networks()
        .get(network)
        .with_context(|| format!("Unable to find network config: {network}"))?
        .to_owned();

    let client = Client::new(network_info.clone()).to_signing_client(signing_key, account_prefix);

    let wasm = read_wasm(ctx.root()?, contract_name, no_wasm_opt)?;
    let store_code_proposal = cosmrs::proto::cosmwasm::wasm::v1::StoreCodeProposal {
        title: title.to_string(),
        description: description.to_string(),
        run_as: client.signer_account_id().to_string(),
        wasm_byte_code: wasm,
        instantiate_permission: None, // TODO: add instantitate permission
    };

    let msg_submit_proposal = MsgSubmitProposal {
        content: Some(Any {
            type_url: "/cosmwasm.wasm.v1.StoreCodeProposal".to_owned(),
            value: store_code_proposal.to_bytes()?,
        }),
        initial_deposit: deposit.into(),
        proposer: client.signer_account_id().to_string(),
    };

    let msg_submit_proposal = Any {
        type_url: "/cosmos.gov.v1beta1.MsgSubmitProposal".to_owned(),
        value: msg_submit_proposal.to_bytes()?,
    };

    block(async {
        let response = client
            .sign_and_broadcast(vec![msg_submit_proposal], gas, "", timeout_height)
            .await?;

        let proposal_id: u64 = response
            .pick("submit_proposal", "proposal_id")
            .to_string()
            .parse()?;

        // TODO: ProposalStoreCodeResponse::from(response)
        let deposit_amount: String = response.pick("proposal_deposit", "amount").to_string();
        let deposit_amount = if deposit_amount.is_empty() {
            "-".to_string()
        } else {
            deposit_amount
        };

        let propose_store_code_response = ProposeStoreCodeResponse {
            proposal_id,
            deposit_amount,
        };

        State::update_state_file(
            network_info.network_variant(),
            ctx.root()?,
            &|s: &State| -> State {
                s.update_proposal_store_code_id(network, contract_name, &proposal_id)
            },
        )?;
        propose_store_code_response.log();

        Ok(propose_store_code_response)
    })
}

#[allow(dead_code)]
pub struct ProposeStoreCodeResponse {
    pub(crate) proposal_id: u64,
    pub(crate) deposit_amount: String,
}

impl OpResponseDisplay for ProposeStoreCodeResponse {
    fn headline() -> &'static str {
        "Store code proposal has been submitted!! 🎉"
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | proposal_id, deposit_amount }
    }
}
