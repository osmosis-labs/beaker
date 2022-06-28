use crate::framework::Context;
use crate::modules::wasm::config::WasmConfig;
use crate::support::template::Template;
use anyhow::Result;
use std::path::PathBuf;

pub fn new<'a, Ctx: Context<'a, WasmConfig>>(
    ctx: &Ctx,
    name: &str,
    version: Option<String>,
    target_dir: Option<PathBuf>,
) -> Result<()> {
    let cfg = ctx.config()?;
    let repo = &cfg.template_repo;
    let version = version.unwrap_or_else(|| "main".to_string());
    let target_dir =
        target_dir.unwrap_or(ctx.root()?.join(PathBuf::from(cfg.contract_dir.as_str())));

    let cw_template = Template::new(name.to_string(), repo.to_owned(), version, None, target_dir);
    cw_template.generate()
}
