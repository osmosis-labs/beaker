use clap::Subcommand;

use crate::{
    framework::Context,
    modules::wasm::{args::BaseTxArgs, WasmConfig},
    support::gas::Gas,
};

#[derive(Subcommand, Debug)]
pub enum ProposalCmd {
    /// Proposal for storing .wasm on chain for later initialization
    StoreCode {
        /// Name of the contract to store
        contract_name: String,

        /// Proposal title
        #[clap(long)]
        title: String,

        /// Proposal decsription
        #[clap(short, long)]
        description: String,

        /// Proposal deposit to activate voting
        #[clap(long)]
        deposit: Option<String>,

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
            title,
            description,
            base_tx_args,
            deposit,
        } => {
            let BaseTxArgs {
                network,
                signer_args,
                gas_args,
                timeout_height,
            }: &BaseTxArgs = base_tx_args;

            super::ops::propose_store_code(
                &ctx,
                contract_name,
                title,
                description,
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
