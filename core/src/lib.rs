use std::{env, path::PathBuf};

use anyhow::{anyhow, Context as ErrContext, Result};
use clap::Subcommand;
use config::Config;

use serde::{Deserialize, Serialize};

pub trait Module<'a, Config, Cmd: Subcommand, Err>
where
    Config: Serialize + Deserialize<'a> + Default,
{
    fn execute<Ctx: Context<'a, Config>>(ctx: Ctx, cmd: &Cmd) -> Result<(), Err>;
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
}

#[macro_export]
macro_rules! config_impl {
    ($key:ident, $cfg:ident) => {
        fn config(&self) -> Result<$cfg> {
            #[derive(Default, Serialize, Deserialize)]
            struct ConfigWrapper {
                $key: $cfg,
            }

            let conf = Config::builder().add_source(Config::try_from(&ConfigWrapper::default())?);
            let conf = match self.config_file_path() {
                Ok(path) => conf.add_source(config::File::from(path)),
                _ => conf,
            };
            conf.build()?
                .try_deserialize::<ConfigWrapper>()
                .with_context(|| "Unable to deserilize configuration.")
                .map(|w| w.$key)
        }
    };
}

#[macro_export]
macro_rules! context {
    ($ctx:ident, config={ $key:ident: $cfg:ident }) => {
        struct $ctx {}
        impl<'a> Context<'a, $cfg> for $ctx {
            protostar_core::config_impl!($key, $cfg);
        }
    };

    ($ctx:ident, config={ $key:ident: $cfg:ident }, config_file=$cfg_file:expr) => {
        struct $ctx {}
        impl<'a> Context<'a, $cfg> for $ctx {
            fn config_file_name(&self) -> String {
                $cfg_file.to_string()
            }

            protostar_core::config_impl!($key, $cfg);
        }
    };

    ($($ctx:ident, config={ $key:ident: $cfg:ident });+) => {
        $(context!($ctx, config = { $key: $cfg });)+
    };

    (config_file=$cfg_file:expr; $($ctx:ident, config={ $key:ident: $cfg:ident });+) => {
        $(context!($ctx, config = { $key: $cfg }, config_file=$cfg_file);)+
    };
}
