use super::{config::TaskConfig, script_mod::wasm};
use crate::framework::{Context, Module};
use anyhow::Result;
use clap::{Arg, Command, Subcommand};
use rhai::{
    exported_module,
    serde::{from_dynamic, to_dynamic},
    Dynamic, Engine, EvalAltResult, Map,
};

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

                let moved_script = script.to_owned();

                let args = args.to_owned();

                engine.register_fn(
                    "match_args",
                    move |arg_defs: Dynamic| -> Result<Map, Box<EvalAltResult>> {
                        let arg_defs = from_dynamic::<Vec<&str>>(&arg_defs)?;

                        let prog_name = format!("beaker task run {}", moved_script);

                        let matches = Command::new(prog_name.clone())
                            .args(arg_defs.clone().into_iter().map(|arg| {
                                Arg::new(arg).long(arg).required(true).takes_value(true)
                            }))
                            .try_get_matches_from(
                                // emulate calling cli command with args
                                vec![prog_name].iter().chain(args.to_owned().iter()),
                            )
                            .map_err(|e| <Box<EvalAltResult>>::from(e.to_string()))?;

                        arg_defs
                            .into_iter()
                            .map(|arg| -> Result<_, Box<EvalAltResult>> {
                                let matched = matches.value_of(arg).unwrap_or("");
                                Ok((arg.into(), to_dynamic(matched)?))
                            })
                            .collect::<Result<Map, _>>()
                    },
                );

                engine.run_file(script_path.clone()).map_err(|e| {
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
