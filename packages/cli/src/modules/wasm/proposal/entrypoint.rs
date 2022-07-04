use std::path::PathBuf;

use clap::Subcommand;

use crate::{
    framework::Context,
    modules::wasm::{args::BaseTxArgs, WasmConfig},
    support::gas::Gas,
};

use super::proposal_struct::StoreCodeProposal;

#[derive(Subcommand, Debug)]
pub enum ProposalCmd {
    /// Proposal for storing .wasm on chain for later initialization
    StoreCode {
        /// Name of the contract to store
        contract_name: String,

        /// Path to proposal file, could be either yaml / toml format.
        #[clap(short, long)]
        proposal: Option<PathBuf>,

        #[clap(flatten)]
        store_code_proposal: StoreCodeProposal,

        #[clap(flatten)]
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
        base_tx_args: BaseTxArgs,
    },
    Query {
        #[clap(subcommand)]
        cmd: ProposalQueryCmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProposalQueryCmd {
    /// Proposal for storing .wasm on chain for later initialization
    StoreCode {
        /// Name of the contract to store
        contract_name: String,

        #[clap(short, long, default_value = "local")]
        network: String,
    },
}

pub fn execute<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: Ctx,
    cmd: &ProposalCmd,
) -> Result<(), anyhow::Error> {
    match cmd {
        ProposalCmd::StoreCode {
            contract_name,
            proposal,
            store_code_proposal,
            base_tx_args,
        } => {
            let proposal = if let Some(p) = proposal {
                let proposal_str = std::fs::read_to_string(p)?;
                let store_code_proposal: StoreCodeProposal =
                    serde_yaml::from_str(proposal_str.as_str())?;
                Some(store_code_proposal)
            } else {
                None
            };

            let store_code_proposal @ StoreCodeProposal { title, deposit, .. } =
                proposal.as_ref().unwrap_or(store_code_proposal);

            let BaseTxArgs {
                network,
                signer_args,
                gas_args,
                timeout_height,
            }: &BaseTxArgs = base_tx_args;

            super::ops::propose_store_code(
                &ctx,
                contract_name,
                title.as_str(),
                store_code_proposal.description_with_metadata()?.as_str(),
                deposit.as_ref().map(|s| s.as_str()).try_into()?,
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
            )?;
            Ok(())
        }
        ProposalCmd::Query { cmd } => match cmd {
            ProposalQueryCmd::StoreCode {
                contract_name,
                network,
            } => {
                super::ops::query_proposal(&ctx, contract_name, network)?;
                Ok(())
            }
        },
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
            )?;
            Ok(())
        }
    }
}
