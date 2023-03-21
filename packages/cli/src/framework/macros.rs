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
        #[derive(Debug, Clone)]
        pub struct $ctx {}
        impl $ctx {
            fn new() -> Self {
                $ctx {}
            }
        }
        impl<'a> Context<'a, $cfg> for $ctx {
            framework::macros::config_impl!($key, $cfg);
        }
    };

    ($ctx:ident, config={ $key:ident: $cfg:ident }, config_file=$cfg_file:expr) => {
        struct $ctx {}
        impl<'a> Context<'a, $cfg> for $ctx {
            fn config_file_name(&self) -> String {
                $cfg_file.to_string()
            }

            framework::macros::config_impl!($key, $cfg);
        }
    };

    ($($ctx:ident, config={ $key:ident: $cfg:ident });+) => {
        $(context!($ctx, config = { $key: $cfg });)+
    };

    (config_file=$cfg_file:expr; $($ctx:ident, config={ $key:ident: $cfg:ident });+) => {
        $(context!($ctx, config = { $key: $cfg }, config_file=$cfg_file);)+
    };
}

pub use config_impl;
