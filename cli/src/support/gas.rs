use anyhow::{Context, Error, Result};
use clap::Parser;
use cosmrs::tx::Fee;
use getset::Getters;

use super::coin::CoinFromStr;
#[derive(Debug, Parser, Getters, Clone)]
#[get = "pub"]
pub struct GasArgs {
    /// Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`
    #[clap(long)]
    gas: String,
    /// Limit to how much gas amount allowed to be consumed
    #[clap(long)]
    gas_limit: u64,
}

impl TryFrom<GasArgs> for Fee {
    type Error = Error;

    fn try_from(value: GasArgs) -> Result<Self, Self::Error> {
        Fee::try_from(&value)
            .with_context(|| format!("Unable to convert GasArgs into Fee: {:?}", &value))
    }
}

impl TryFrom<&GasArgs> for Fee {
    type Error = Error;

    fn try_from(value: &GasArgs) -> Result<Self, Self::Error> {
        let amount = value.gas.parse::<CoinFromStr>()?.inner().to_owned();
        Ok(Fee::from_amount_and_gas(amount, value.gas_limit))
    }
}
