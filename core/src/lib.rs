use clap::Subcommand;
use serde::{Deserialize, Serialize};

pub trait Module<'a, Config, Cmd: Subcommand, Err>
where
    Config: Serialize + Deserialize<'a> + Default,
{
    fn execute(&self, cfg: &Config, cmd: &Cmd) -> Result<(), Err>;
}
