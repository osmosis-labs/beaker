use clap::Parser;

use crate::support::{gas::GasArgs, signer::SignerArgs};

#[derive(Debug, Parser, Clone)]
pub struct BaseTxArgs {
    /// Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config
    #[clap(short, long, default_value = "local")]
    pub network: String,

    #[clap(flatten)]
    pub gas_args: GasArgs,

    #[clap(flatten)]
    pub signer_args: SignerArgs,

    /// Specifies a block timeout height to prevent the tx from being committed past a certain height
    #[clap(short, long, default_value = "0")]
    pub timeout_height: u32,

    /// Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain.
    /// This is useful if there is an account sequence mismatch.
    #[clap(short, long)]
    pub account_sequence: Option<u64>,
}
