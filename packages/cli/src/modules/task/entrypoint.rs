use super::{config::TaskConfig, script_mod::wasm};
use crate::framework::{Context, Module};
use anyhow::Result;
use clap::Subcommand;
use rhai::{exported_module, serde::to_dynamic, Dynamic, Engine, Scope};

#[derive(Subcommand, Debug)]
#[clap(trailing_var_arg = true)]
pub enum TaskCmd {
    Run {
        script: String,

        #[clap(multiple_values = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

pub struct TaskModule {}

impl<'a> Module<'a, TaskConfig, TaskCmd, anyhow::Error> for TaskModule {
    fn execute<Ctx: Context<'a, TaskConfig>>(ctx: Ctx, cmd: &TaskCmd) -> Result<(), anyhow::Error> {
        let root = ctx.root()?;
        let config = ctx.config()?;

        match cmd {
            TaskCmd::Run { script, args } => {
                let mut engine = Engine::new();

                let wasm = exported_module!(wasm::commands);

                engine.register_static_module("wasm", wasm.into());

                let script_path = root
                    .join(config.tasks_path)
                    .join(format!("{}.rhai", script));

                let mut scope = Scope::new();

                // TODO: using [clap-serde](https://github.com/aobatact/clap-serde#json) to
                // allow defining expected args from .rhai file
                scope.push(
                    "args",
                    args.iter()
                        .map(|s| to_dynamic(s).unwrap())
                        .collect::<Vec<Dynamic>>(),
                );

                engine
                    .run_file_with_scope(&mut scope, script_path.clone())
                    .map_err(|e| {
                        anyhow::anyhow!(
                            "Failed to run script `{}` ({}):\n{}",
                            script,
                            script_path.display(),
                            e.to_string()
                        )
                    })?;
                Ok(())
            }
        }
    }
}
