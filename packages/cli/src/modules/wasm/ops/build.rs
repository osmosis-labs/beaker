use std::{env, process::Command};

use anyhow::Result;

use crate::support::command::run_command;
use crate::{framework::Context, modules::wasm::WasmConfig};

pub fn build<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    no_wasm_opt: &bool,
    aarch64: &bool,
) -> Result<()> {
    let root = ctx.root()?;

    let wp_name = root.file_name().unwrap().to_str().unwrap(); // handle properly

    env::set_current_dir(&root)?;

    let root_dir_str = root.to_str().unwrap();

    let list_installed_target = Command::new("rustup")
        .arg("target")
        .arg("list")
        .arg("--installed")
        .output()?;
    let installed_target = String::from_utf8(list_installed_target.stdout)?;

    if !installed_target
        .split('\n')
        .any(|t| t == "wasm32-unknown-unknown")
    {
        run_command(
            Command::new("rustup")
                .arg("target")
                .arg("add")
                .arg("wasm32-unknown-unknown"),
        )?;
    };

    let _build = run_command(
        Command::new("cargo")
            .env("RUSTFLAGS", "-C link-arg=-s")
            .arg("build")
            .arg("--release")
            .arg("--target")
            .arg("wasm32-unknown-unknown"),
    )?;

    if !*no_wasm_opt {
        println!("Optimizing wasm...");
        let optimizer_version = ctx.config()?.optimizer_version;

        let arch_suffix = if *aarch64 { "-arm64" } else { "" };

        let _optim = run_command(Command::new("docker").args(&[
            "run",
            "--rm",
            "-v",
            format!("{root_dir_str}:/code").as_str(),
            "--mount",
            format!("type=volume,source={wp_name}_cache,target=/code/target").as_str(),
            "--mount",
            "type=volume,source=registry_cache,target=/usr/local/cargo/registry",
            format!("cosmwasm/workspace-optimizer{arch_suffix}:{optimizer_version}").as_str(),
        ]))?;
    }

    Ok(())
}
