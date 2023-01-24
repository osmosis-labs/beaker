use crate::attrs_format;
use crate::support::cosmos::ResponseValuePicker;
use crate::support::future::block;
use crate::support::gas::Gas;
use crate::support::ops_response::OpResponseDisplay;
use crate::support::state::State;
use crate::{framework::Context, modules::wasm::WasmConfig, support::cosmos::Client};
use anyhow::anyhow;
use anyhow::{Context as _, Result};
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tx::MessageExt;
use cosmrs::Any;
use std::str::FromStr;
use std::vec;

#[allow(clippy::too_many_arguments)]
pub fn vote<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    contract_name: &str,
    option: &str,
    network: &str,
    gas: &Gas,
    timeout_height: &u32,
    signing_key: SigningKey,
    account_sequence: &Option<u64>,
) -> Result<VoteResponse> {
    let global_config = ctx.global_config()?;
    let account_prefix = global_config.account_prefix().as_str();

    let network_info = global_config
        .networks()
        .get(network)
        .with_context(|| format!("Unable to find network config: {network}"))?
        .to_owned();

    let client = Client::new(network_info.clone()).to_signing_client(signing_key, account_prefix);

    let state = State::load_by_network(network_info, ctx.root()?)?;
    let proposal_id = state
        .get_ref(network, contract_name)?
        .proposal()
        .store_code()
        .with_context(|| format!("Unable to retrieve proposal_id for {contract_name}"))?;

    let option = option.parse::<VoteOptionImpl>()?;
    let option = cosmrs::proto::cosmos::gov::v1beta1::VoteOption::from(option);

    let msg_vote = cosmrs::proto::cosmos::gov::v1beta1::MsgVote {
        proposal_id,
        voter: client.signer_account_id().to_string(),
        option: option.into(),
    };

    let msg_vote = Any {
        type_url: "/cosmos.gov.v1beta1.MsgVote".to_owned(),
        value: msg_vote.to_bytes()?,
    };

    block(async {
        let response = client
            .sign_and_broadcast(vec![msg_vote], gas, "", timeout_height, account_sequence)
            .await?;

        let proposal_id: u64 = response.pick("proposal_vote", "proposal_id").parse()?;

        let vote_response = VoteResponse { proposal_id };

        vote_response.log();

        Ok(vote_response)
    })
}

struct VoteOptionImpl(cosmrs::proto::cosmos::gov::v1beta1::VoteOption);

impl From<cosmrs::proto::cosmos::gov::v1beta1::VoteOption> for VoteOptionImpl {
    fn from(v: cosmrs::proto::cosmos::gov::v1beta1::VoteOption) -> Self {
        VoteOptionImpl(v)
    }
}

impl From<VoteOptionImpl> for cosmrs::proto::cosmos::gov::v1beta1::VoteOption {
    fn from(v: VoteOptionImpl) -> Self {
        let VoteOptionImpl(vo) = v;
        vo
    }
}

impl FromStr for VoteOptionImpl {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use cosmrs::proto::cosmos::gov::v1beta1::VoteOption;

        match s {
            "yes" => Ok(VoteOption::Yes.into()),
            "no" => Ok(VoteOption::No.into()),
            "no_with_veto" => Ok(VoteOption::NoWithVeto.into()),
            "abstain" => Ok(VoteOption::Abstain.into()),
            o => Err(anyhow!("Invalid vote option: {o}")),
        }
    }
}

pub struct VoteResponse {
    pub proposal_id: u64,
}

impl OpResponseDisplay for VoteResponse {
    fn headline() -> &'static str {
        "Voted successfully!! 🎉"
    }
    fn attrs(&self) -> Vec<String> {
        attrs_format! { self | proposal_id }
    }
}
