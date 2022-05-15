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

    let default_config_file_name = "Protostar.toml";
    let config_file_name = ctx.config_file_name();
    fs::rename(
        root_dir.join::<PathBuf>(default_config_file_name.into()),
        root_dir.join::<PathBuf>(config_file_name.clone().into()),
    )
    .with_context(|| format!("Unable to rename {default_config_file_name} to {config_file_name}"))
}
