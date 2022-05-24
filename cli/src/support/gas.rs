use clap::Parser;
use getset::Getters;

#[derive(Debug, Parser, Getters)]
#[get = "pub"]
pub struct GasArgs {
    /// Amount of coin willing to pay as gas
    #[clap(long)]
    gas: u64,
    /// Limit to how much gas amount allowed to be consumed
    #[clap(long)]
    gas_limit: u64,
}
