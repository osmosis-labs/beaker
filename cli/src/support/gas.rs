use anyhow::{Context, Error, Result};
use clap::Parser;
use cosmrs::{tx::Fee, Coin};
use getset::Getters;
#[derive(Debug, Parser, Getters, Clone, Copy)]
#[get = "pub"]
pub struct GasArgs {
    /// Amount of coin willing to pay as gas
    #[clap(long)]
    gas: u64,
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
        let amount = Coin {
            amount: value.gas.to_owned().into(),
            denom: "uosmo".parse().unwrap(),
        };
        Ok(Fee::from_amount_and_gas(amount, value.gas_limit))
    }
}
