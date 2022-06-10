use crate::support::future::block;
use crate::support::state::State;
use crate::vars_format;
use crate::{framework::Context, modules::wasm::WasmConfig, support::cosmos::Client};
use anyhow::{Context as _, Result};
use cosmos_sdk_proto::cosmos::gov::v1beta1::{Proposal, ProposalStatus, TallyResult};
use cosmrs::bip32::secp256k1::pkcs8::der::DateTime;
use std::time::Duration;
use std::vec;

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
            .proposal(&wasm_ref.proposal().store_code().with_context(|| {
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
