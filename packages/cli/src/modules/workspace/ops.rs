use super::config::WorkspaceConfig;
use crate::framework::Context;
use anyhow::{Context as _, Result};
use std::{fs, path::PathBuf};

pub fn new<'a, Ctx: Context<'a, WorkspaceConfig>>(
    ctx: Ctx,
    name: &String,
    branch: &Option<String>,
    target_dir: &Option<PathBuf>,
) -> Result<()> {
    let template = ctx
        .config()?
        .template
        .with_name(Some(name.to_string()))
        .with_branch(branch.to_owned())
        .with_target_dir(target_dir.to_owned());

    template.generate()?;

    let root_dir = template.target_dir().join::<PathBuf>(name.into());

    let default_config_file_name = "Beaker.toml";
    let config_file_name = ctx.config_file_name();
    fs::rename(
        root_dir.join::<PathBuf>(default_config_file_name.into()),
        root_dir.join::<PathBuf>(config_file_name.clone().into()),
    )
    .with_context(|| {
        format!("Unable to rename {default_config_file_name} to {config_file_name}")
    })?;

    let frontend_dir = root_dir.join("frontend");

    if frontend_dir.exists() {
        fs::rename(
            frontend_dir.join(".env.local.example"),
            frontend_dir.join(".env.local"),
        )
        .with_context(|| "Unable to rename `.env.local.example` to `.env.local`")?;

        // symlink .beaker to frontend
        std::env::set_current_dir(root_dir.join("frontend"))?;
        std::os::unix::fs::symlink("../.beaker", ".beaker")
        .with_context(|| "Currently not support symbolic link on non-unix system, if you are on windows, please consider using wsl.")?;
    }
    Ok(())
}
