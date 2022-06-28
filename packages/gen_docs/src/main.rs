use beaker::{Cli, ConsoleConfig, GlobalConfig, WasmConfig, WorkspaceConfig};
use clap::CommandFactory;
use serde::Serialize;
use std::io::Write;
use std::path::Path;
use std::vec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Cli::command();

    let docs_path = Path::new("docs");
    if docs_path.exists() {
        std::fs::remove_dir_all(docs_path)?;
    }

    let command_path = docs_path.join("commands");
    gen_docs::command::generate_command_doc(&app, command_path.clone())?;
    std::fs::rename(
        command_path.join(format!("{}.md", app.get_name())),
        command_path.join("README.md"),
    )?;

    let config_path = docs_path.join("config");
    gen_docs::generate_config_doc!(config_path, {
        #[no_wrap] global: GlobalConfig,
        workspace: WorkspaceConfig,
        wasm: WasmConfig,
        console: ConsoleConfig,
    });

    Ok(())
}
