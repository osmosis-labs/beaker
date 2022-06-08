use std::io::Read;
use std::time::Duration;
use std::vec;
use std::{fs::File, future::Future, io::BufReader};

use anyhow::{Context as _, Result};
use cosmos_sdk_proto::cosmos::gov::v1beta1::{
    MsgSubmitProposal, Proposal, ProposalStatus, TallyResult,
};
use cosmrs::bip32::secp256k1::pkcs8::der::DateTime;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::{tx::Fee, Any};

use crate::modules::wasm::proposal::reponse::ProposeStoreCodeResponse;
use crate::support::coin::CoinFromStr;
use crate::support::cosmos::ResponseValuePicker;
use crate::support::ops_response::OpResponseDisplay;
use crate::support::state::State;
use crate::vars_format;
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

    let network_info = global_config
        .networks()
        .get(network)
        .with_context(|| format!("Unable to find network config: {network}"))?
        .to_owned();

    let client = Client::new(network_info.clone()).to_signing_client(signing_key, account_prefix);

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

pub fn query_proposal<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    network: &str,
) -> Result<Proposal> {
    let global_config = ctx.global_config()?;

    let network_info = global_config
        .networks()
        .get(network)
        .with_context(|| format!("Unable to find network config: {network}"))?
        .to_owned();

    let client = Client::new(network_info.clone());

    let state = State::load_by_network(network_info, ctx.root()?)?;
    let wasm_ref = state.get_ref(network, contract_name)?;

    block(async {
        let res = client
            .proposal(&wasm_ref.proposal_store_code_id().with_context(|| {
                format!(
            "Proposal store code not found for contract `{contract_name}` on network `{network}`"
        )
            })?)
            .await?;

        use cosmos_sdk_proto::cosmwasm::wasm::v1::StoreCodeProposal;
        use prost::Message;

        let Proposal {
            proposal_id,
            content,
            status,
            total_deposit,
            final_tally_result,
            submit_time,
            deposit_end_time,
            voting_start_time,
            voting_end_time,
        } = res.clone();

        let status = ProposalStatus::from_i32(status).unwrap();
        let status = match status {
            ProposalStatus::DepositPeriod => "DepositPeriod",
            ProposalStatus::Unspecified => "Unspecified",
            ProposalStatus::VotingPeriod => "VotingPeriod",
            ProposalStatus::Passed => "Passed",
            ProposalStatus::Rejected => "Rejected",
            ProposalStatus::Failed => "Failed",
        };

        let TallyResult {
            yes,
            abstain,
            no,
            no_with_veto,
        } = final_tally_result.unwrap();

        let StoreCodeProposal {
            title,
            description,
            run_as,
            ..
        } = StoreCodeProposal::decode(content.unwrap().value.as_slice())?;

        let total_deposit = total_deposit
            .iter()
            .map(|c| format!("{}{}", c.amount, c.denom))
            .collect::<Vec<String>>()
            .join(",");

        let min_deposit = client
            .gov_params_deposit()
            .await?
            .min_deposit
            .iter()
            .map(|c| format!("{}{}", c.amount, c.denom))
            .collect::<Vec<String>>()
            .join(",");

        let total_deposit = format!("{total_deposit} (min_deposit: {min_deposit})");

        let datetime_str = |seconds: i64, nanos: i32| {
            if let Ok(d) = DateTime::from_unix_duration(Duration::new(seconds as u64, nanos as u32))
            {
                format!("{d}")
            } else {
                "–".to_string()
            }
        };

        let submit_time = {
            let ts = submit_time.unwrap();
            datetime_str(ts.seconds, ts.nanos)
        };
        let deposit_end_time = {
            let ts = deposit_end_time.unwrap();
            datetime_str(ts.seconds, ts.nanos)
        };
        let voting_start_time = {
            let ts = voting_start_time.unwrap();
            datetime_str(ts.seconds, ts.nanos)
        };
        let voting_end_time = {
            let ts = voting_end_time.unwrap();
            datetime_str(ts.seconds, ts.nanos)
        };

        println!(
            "{}",
            vec![
                vars_format!(
                    "Proposal found!",
                    proposal_id,
                    title,
                    description,
                    run_as,
                    total_deposit,
                    status
                ),
                vars_format!("Tally Result", yes, no, no_with_veto, abstain),
                vars_format!(
                    "Time",
                    submit_time,
                    deposit_end_time,
                    voting_start_time,
                    voting_end_time
                ),
            ]
            .concat()
            .join("\n")
        );

        Ok(res)
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
