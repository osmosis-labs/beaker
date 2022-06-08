use clap::Subcommand;
use cosmrs::tx::Fee;

use crate::{
    framework::Context,
    modules::wasm::{args::BaseTxArgs, WasmConfig},
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

        /// Proposal decsription
        #[clap(long)]
        deposit: Option<String>,

        #[clap(flatten)]
        base_tx_args: BaseTxArgs,
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
                deposit.as_ref().map(|s| s.as_str()),
                network,
                &Fee::try_from(gas_args)?,
                timeout_height,
                signer_args.private_key(&ctx.global_config()?)?,
            )?;
            Ok(())
        }
    }
}
