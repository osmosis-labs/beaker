use super::Context;
use anyhow::Result;
use clap::Subcommand;
use serde::{Deserialize, Serialize};

pub trait Module<'a, Config, Cmd: Subcommand, Err>
where
    Config: Serialize + Deserialize<'a> + Default,
{
    fn execute<Ctx: Context<'a, Config>>(ctx: Ctx, cmd: &Cmd) -> Result<(), Err>;
}
