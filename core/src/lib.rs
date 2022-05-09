use clap::Subcommand;
use serde::{Deserialize, Serialize};

pub trait Module<'a, Cmd: Subcommand, Err>: Serialize + Deserialize<'a> + Default {
    fn execute(self: &Self, cmd: &Cmd) -> Result<(), Err>;
}
