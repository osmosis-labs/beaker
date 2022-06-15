use std::str::FromStr;

use anyhow::{Context, Error, Result};
use clap::Parser;
use cosmrs::{tx::Fee, Denom};
use getset::Getters;
use regex::Regex;

use super::coin::CoinFromStr;
#[derive(Debug, Parser, Getters, Clone)]
#[get = "pub"]
pub struct GasArgs {
    /// Coin (amount and denom) you are willing to pay as gas eg. `1000uosmo`
    #[clap(long, default_value = "auto")]
    gas: String,
    /// Limit to how much gas amount allowed to be consumed
    #[clap(long, default_value = "0")]
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

#[derive(Debug, Clone)]
pub struct GasPrice {
    pub amount: f64,
    pub denom: Denom,
}

impl FromStr for GasPrice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+\.?\d*)(.+)$").unwrap();
        let caps = re
            .captures(s)
            .with_context(|| format!("Unable to parse `{s}` as Coin."))?;

        let gas_price = GasPrice {
            amount: caps
                .get(1)
                .with_context(|| format!("`{s}` does not contain valid amount"))?
                .as_str()
                .parse()
                .unwrap(),
            denom: caps
                .get(2)
                .with_context(|| format!("`{s}` does not contain valid denom"))?
                .as_str()
                .parse()
                .unwrap(),
        };

        Ok(gas_price)
    }
}

#[derive(Debug, Clone)]
pub enum Gas {
    Specified(Fee),
    Auto {
        gas_price: GasPrice,
        gas_adjustment: f32,
    },
}

impl Gas {
    pub fn from_args(args: &GasArgs, gas_price: &str, gas_adjustment: &f32) -> Result<Self> {
        if args.gas() == "auto" {
            Ok(Self::Auto {
                gas_price: gas_price.parse()?,
                gas_adjustment: gas_adjustment.to_owned(),
            })
        } else {
            Ok(Self::Specified(Fee::try_from(args)?))
        }
    }
}
