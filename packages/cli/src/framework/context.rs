use anyhow::{anyhow, Context as ErrContext, Result};
use config::Config;
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};

use super::config::GlobalConfig;

pub trait Context<'a, Cfg>: Send
where
    Cfg: Serialize + Deserialize<'a> + Default,
{
    fn config_file_name(&self) -> String {
        "Tesseract.toml".to_string()
    }

    fn config_file_path(&self) -> Result<PathBuf> {
        let curr = env::current_dir()?;
        let it = curr.ancestors();

        for p in it {
            let p = p.join(self.config_file_name());
            if p.exists() {
                return Ok(p);
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
            .ok_or_else(|| anyhow!("Already at root dir"))
    }

    fn config(&self) -> Result<Cfg> {
        let conf = Config::builder().add_source(Config::try_from(&Cfg::default())?);
        let conf = match self.config_file_path() {
            Ok(path) => conf.add_source(config::File::from(path)),
            _ => conf,
        };
        conf.build()?
            .try_deserialize::<Cfg>()
            .with_context(|| "Unable to deserialize configuration.")
    }

    fn global_config(&self) -> Result<GlobalConfig> {
        let conf = Config::builder().add_source(Config::try_from(&GlobalConfig::default())?);
        let conf = match self.config_file_path() {
            Ok(path) => conf.add_source(config::File::from(path)),
            _ => conf,
        };
        conf.build()?
            .try_deserialize::<GlobalConfig>()
            .with_context(|| "Unable to deserialize configuration.")
    }
}
