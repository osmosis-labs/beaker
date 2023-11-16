use crate::framework::Context;
use crate::modules::wasm::config::WasmConfig;
use crate::support::template::Template;
use anyhow::{Context as _, Result};
use console::{style, Emoji};
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::PathBuf;

pub static SHRUG: Emoji<'_, '_> = Emoji("🤷  ", "");

pub fn new<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    name: &str,
    version: Option<String>,
    target_dir: Option<PathBuf>,
    template: Option<String>,
) -> Result<()> {
    let cfg = ctx.config()?;
    let templates = cfg.template_repos.keys().collect::<Vec<_>>();
    let version = version.unwrap_or_else(|| "main".to_string());
    let target_dir =
        target_dir.unwrap_or(ctx.root()?.join(PathBuf::from(cfg.contract_dir.as_str())));

    let repo = if let Some(template) = template {
        cfg.template_repos
            .get(&template)
            .ok_or_else(|| anyhow::anyhow!("Unable to get template repository"))?
    } else {
        let template_idx = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "{} {}",
                SHRUG,
                style("Which template would you like to use?").bold()
            ))
            .items(&templates)
            .default(0)
            .interact()
            .with_context(|| "Canceled")?;

        cfg.template_repos
            .get(templates[template_idx].as_str())
            .ok_or_else(|| anyhow::anyhow!("Unable to get template repository"))?
    };

    let cw_template = Template::new(
        name.to_string(),
        repo.to_string(),
        version,
        None,
        target_dir,
    );
    cw_template.generate()
}
