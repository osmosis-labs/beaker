use clap::Parser;
use serde::Deserialize;

use crate::support::{gas::GasArgs, signer::SignerArgs};

#[derive(Debug, Parser, Clone, Deserialize)]
pub struct BaseTxArgs {
    /// Name of the network to broadcast transaction to, the actual endpoint / chain-id are defined in config
    #[clap(short, long, default_value = "local")]
    #[serde(default = "default_value::network")]
    pub network: String,

    #[clap(flatten)]
    #[serde(flatten)]
    pub gas_args: GasArgs,

    #[clap(flatten)]
    #[serde(flatten)]
    pub signer_args: SignerArgs,

    /// Specifies a block timeout height to prevent the tx from being committed past a certain height
    #[clap(short, long, default_value = "0")]
    #[serde(default = "default_value::timeout_height")]
    pub timeout_height: u32,

    /// Account sequence number to use for the transaction, if not provided, sequence will be fetched from the chain.
    /// This is useful if there is an account sequence mismatch.
    #[clap(short, long)]
    pub account_sequence: Option<u64>,
}

mod default_value {
    pub(crate) fn network() -> String {
        "local".to_string()
    }

    // timeout_height is 0 by default
    pub(crate) fn timeout_height() -> u32 {
        0
    }
}
