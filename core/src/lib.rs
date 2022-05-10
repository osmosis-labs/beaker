use std::{env, path::PathBuf};

use anyhow::{anyhow, Context as ErrContext, Result};
use clap::Subcommand;
use config::{builder::DefaultState, Config, ConfigBuilder};

use serde::{Deserialize, Serialize};

pub trait Module<'a, Config, Cmd: Subcommand, Err>
where
    Config: Serialize + Deserialize<'a> + Default,
{
    // fn execute_<Ctx: Context<'a, Config>>(&self, ctx: Ctx, cmd: &Cmd) -> Result<(), Err>;
    fn execute(&self, cfg: &Config, cmd: &Cmd) -> Result<(), Err>;
}

pub trait Context<'a, Cfg>
where
    Cfg: Serialize + Deserialize<'a> + Default,
{
    fn config_file_name(&self) -> String {
        "Protostar.toml".to_string()
    }

    fn config_file_path(&self) -> Result<PathBuf> {
        let curr = env::current_dir()?;
        let mut it = curr.ancestors();

        while let Some(p) = it.next() {
            let p = p.join(self.config_file_name());
            if p.exists() {
                return Ok(p.to_path_buf());
            }
        }

        Err(anyhow!(
            "Config file `{}` not found in all the ancestor paths",
            self.config_file_name()
        ))
    }

    fn root(&self) -> Result<PathBuf> {
        self.config_file_path()?
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or(anyhow!("Already at root dir"))
    }

    fn config(&self) -> Result<Cfg> {
        let conf = Config::builder().add_source(Config::try_from(&Cfg::default())?);
        let conf = match self.config_file_path() {
            Ok(path) => conf.add_source(config::File::from(path)),
            _ => conf,
        };
        conf.build()?
            .try_deserialize::<Cfg>()
            .with_context(|| "Unable to deserilize configuration.")
    }
}
