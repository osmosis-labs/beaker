use clap::Subcommand;

pub trait Module<Cmd: Subcommand, Err> {
    fn execute(self: &Self, cmd: &Cmd) -> Result<(), Err>;
}
