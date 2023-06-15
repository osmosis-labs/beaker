use anyhow::Result;
use clap::Subcommand;
use serde::Deserialize;
use std::path::PathBuf;

use crate::{
    framework::Context,
    modules::wasm::{args::BaseTxArgs, WasmConfig},
    support::gas::Gas,
};

use super::{
    ops::{propose::ProposeStoreCodeResponse, query::QueryProposalResponse, vote::VoteResponse},
    proposal_struct::StoreCodeProposal,
};

#[derive(Subcommand, Debug, Deserialize)]
pub enum ProposalCmd {
    /// Proposal for storing .wasm on chain for later initialization
    StoreCode {
        /// Name of the contract to store
        contract_name: String,

        /// Restricting the code to be able to instantiate/migrate only by given address, no restriction by default
        #[clap(long)]
        permit_instantiate_only: Option<String>,

        /// Path to proposal file, could be either yaml / toml format.
        #[clap(short, long)]
        proposal: Option<PathBuf>,

        #[clap(flatten)]
        #[serde(flatten)]
        store_code_proposal: StoreCodeProposal,

        #[clap(flatten)]
        #[serde(flatten)]
        base_tx_args: BaseTxArgs,
    },
    /// Vote for proposal
    Vote {
        /// Name of the contract to store
        contract_name: String,

        /// Vote option, one of: yes, no, no_with_veto, abstain
        #[clap(short, long)]
        option: String,

        #[clap(flatten)]
        #[serde(flatten)]
        base_tx_args: BaseTxArgs,
    },
    Query {
        #[clap(subcommand)]
        cmd: ProposalQueryCmd,
    },
}

#[derive(Subcommand, Debug, Deserialize)]
pub enum ProposalQueryCmd {
    /// Proposal for storing .wasm on chain for later initialization
    StoreCode {
        /// Name of the contract to store
        contract_name: String,

        #[clap(short, long, default_value = "local")]
        #[serde(default = "default_value::network")]
        network: String,
    },
}

mod default_value {
    pub fn network() -> String {
        "local".to_string()
    }
}

pub fn execute<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: Ctx,
    cmd: &ProposalCmd,
) -> Result<(), anyhow::Error> {
    match cmd {
        cmd @ ProposalCmd::StoreCode { .. } => store_code(ctx, cmd).map(|_| ()),
        cmd @ ProposalCmd::Query { .. } => query(ctx, cmd).map(|_| ()),
        cmd @ ProposalCmd::Vote { .. } => vote(ctx, cmd).map(|_| ()),
    }
}

pub(crate) fn store_code<'a>(
    ctx: impl Context<'a, WasmConfig>,
    cmd: &ProposalCmd,
) -> Result<ProposeStoreCodeResponse> {
    match cmd {
        ProposalCmd::StoreCode {
            contract_name,
            permit_instantiate_only,
            proposal,
            store_code_proposal,
            base_tx_args,
        } => {
            let proposal = if let Some(p) = proposal {
                let proposal_str = std::fs::read_to_string(p)?;
                let extention_error_msg = "Extension must be one of `yaml`, `yml` or `toml`";
                let ext = p.extension().expect(extention_error_msg);
                let store_code_proposal: StoreCodeProposal = if ext == "yaml" || ext == "yml" {
                    serde_yaml::from_str(proposal_str.as_str())?
                } else if ext == "toml" {
                    toml::from_str(proposal_str.as_str())?
                } else {
                    panic!("{}", extention_error_msg);
                };
                Some(store_code_proposal)
            } else {
                None
            };

            let StoreCodeProposal {
                title,
                deposit,
                unpin_code,
                description,
            } = proposal.as_ref().unwrap_or(store_code_proposal);

            let BaseTxArgs {
                network,
                signer_args,
                gas_args,
                timeout_height,
                account_sequence,
            }: &BaseTxArgs = base_tx_args;

            super::ops::propose_store_code(
                &ctx,
                contract_name,
                title.as_str(),
                description.as_str(),
                deposit.as_ref().map(|s| s.as_str()).try_into()?,
                *unpin_code,
                network,
                {
                    let global_conf = ctx.global_config()?;
                    &Gas::from_args(
                        gas_args,
                        global_conf.gas_price(),
                        global_conf.gas_adjustment(),
                    )?
                },
                permit_instantiate_only,
                timeout_height,
                signer_args.private_key(&ctx.global_config()?)?,
                account_sequence,
            )
        }
        _ => unimplemented!(),
    }
}

pub(crate) fn vote<'a>(
    ctx: impl Context<'a, WasmConfig>,
    cmd: &ProposalCmd,
) -> Result<VoteResponse> {
    match cmd {
        ProposalCmd::Vote {
            contract_name,
            option,
            base_tx_args,
        } => {
            let BaseTxArgs {
                network,
                signer_args,
                gas_args,
                timeout_height,
                account_sequence,
            }: &BaseTxArgs = base_tx_args;

            super::ops::vote(
                &ctx,
                contract_name,
                option,
                network,
                {
                    let global_conf = ctx.global_config()?;
                    &Gas::from_args(
                        gas_args,
                        global_conf.gas_price(),
                        global_conf.gas_adjustment(),
                    )?
                },
                timeout_height,
                signer_args.private_key(&ctx.global_config()?)?,
                account_sequence,
            )
        }
        _ => unimplemented!(),
    }
}

pub(crate) fn query<'a>(
    ctx: impl Context<'a, WasmConfig>,
    cmd: &ProposalCmd,
) -> Result<QueryProposalResponse> {
    match cmd {
        ProposalCmd::Query { cmd } => match cmd {
            ProposalQueryCmd::StoreCode {
                contract_name,
                network,
            } => super::ops::query_proposal(&ctx, contract_name, network),
        },
        _ => unimplemented!(),
    }
}
