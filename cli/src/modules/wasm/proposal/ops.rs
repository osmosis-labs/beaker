use std::io::Read;
use std::vec;
use std::{fs::File, future::Future, io::BufReader};

use anyhow::{Context as _, Result};
use cosmos_sdk_proto::cosmos::gov::v1beta1::MsgSubmitProposal;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::{tx::Fee, Any};

use crate::modules::wasm::proposal::reponse::ProposeStoreCodeResponse;
use crate::support::coin::CoinFromStr;
use crate::support::cosmos::ResponseValuePicker;
use crate::support::ops_response::OpResponseDisplay;
use crate::{framework::Context, modules::wasm::WasmConfig, support::cosmos::Client};

pub trait MessageExt: prost::Message {
    /// Serialize this protobuf message as a byte vector.
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

impl<M> MessageExt for M
where
    M: prost::Message,
{
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        prost::Message::encode(self, &mut bytes)?;
        Ok(bytes)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn propose_store_code<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    title: &str,
    description: &str,
    deposit: Option<&str>,
    network: &str,
    fee: &Fee,
    timeout_height: &u32,
    signing_key: SigningKey,
) -> Result<ProposeStoreCodeResponse> {
    let global_config = ctx.global_config()?;
    let account_prefix = global_config.account_prefix().as_str();

    let client = Client::new(
        global_config
            .networks()
            .get(network)
            .with_context(|| format!("Unable to find network config: {network}"))?
            .to_owned(),
    )
    .to_signing_client(signing_key, account_prefix);

    let wasm = read_wasm(ctx, contract_name)?;
    let store_code_proposal = cosmrs::proto::cosmwasm::wasm::v1::StoreCodeProposal {
        title: title.to_string(),
        description: description.to_string(),
        run_as: client.signer_account_id().to_string(),
        wasm_byte_code: wasm,
        instantiate_permission: None, // TODO: add instantitate permission
    };

    let deposit = if let Some(d) = deposit {
        vec![d.parse::<CoinFromStr>()?.inner().into()]
    } else {
        vec![]
    };

    let msg_submit_proposal = MsgSubmitProposal {
        content: Some(Any {
            type_url: "/cosmwasm.wasm.v1.StoreCodeProposal".to_owned(),
            value: store_code_proposal.to_bytes()?,
        }),
        initial_deposit: deposit,
        proposer: client.signer_account_id().to_string(),
    };

    let msg_submit_proposal = Any {
        type_url: "/cosmos.gov.v1beta1.MsgSubmitProposal".to_owned(),
        value: msg_submit_proposal.to_bytes()?,
    };

    block(async {
        let response = client
            .sign_and_broadcast(vec![msg_submit_proposal], fee.clone(), "", timeout_height)
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

        // TODO: Update state
        // State::update_state_file(ctx.root()?, &|s: &State| -> State {
        //     s.update_code_id(network, contract_name, &code_id)
        // })?;
        propose_store_code_response.log();

        Ok(propose_store_code_response)
    })
}

// TODO: Refactor this to share module
fn block<F: Future>(future: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future)
}
fn read_wasm<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
) -> Result<Vec<u8>, anyhow::Error> {
    let wasm_path = ctx
        .root()?
        .as_path()
        .join("artifacts")
        .join(format!("{contract_name}.wasm"));
    let wasm_path_str = &wasm_path.as_os_str().to_string_lossy();
    let f = File::open(&wasm_path).with_context(|| {
        format!(
        "`{wasm_path_str}` not found, please build and optimize the contract before store code`"
    )
    })?;
    let mut reader = BufReader::new(f);
    let mut wasm = Vec::new();
    reader.read_to_end(&mut wasm)?;
    Ok(wasm)
}
