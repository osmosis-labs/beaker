use clap::Parser;

use crate::support::{gas::GasArgs, signer::SignerArgs};

#[derive(Debug, Parser, Clone)]
pub struct BaseTxArgs {
    #[clap(short, long, default_value = "local")]
    pub network: String,

    #[clap(flatten)]
    pub gas_args: GasArgs,

    #[clap(flatten)]
    pub signer_args: SignerArgs,

    /// Specifies a block timeout height to prevent the tx from being committed past a certain height
    #[clap(short, long, default_value = "0")]
    pub timeout_height: u32,
}
