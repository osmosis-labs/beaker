use tesseract::{execute, Cli};
use clap::Parser;

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    execute(&cli.command)
}
