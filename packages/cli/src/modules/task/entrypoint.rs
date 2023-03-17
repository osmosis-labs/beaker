use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use super::{
    config::TaskConfig,
    script_mod::{wasm, wasm_proposal},
};
use crate::framework::{Context, Module};
use anyhow::Result;
use clap::{Arg, Command, Subcommand};
use regex::Regex;
use rhai::packages::Package;
use rhai::{
    exported_module,
    serde::{from_dynamic, to_dynamic},
    Dynamic, Engine, EvalAltResult, Map,
};
use rhai_fs::FilesystemPackage;

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
        let global_config = ctx.global_config()?;

        match cmd {
            TaskCmd::Run { script, args } => {
                let mut engine = Engine::new();

                // register filesystem package
                let package = FilesystemPackage::new();
                package.register_into_engine_as(&mut engine, "fs");

                // register wasm & wasm_proposal module
                let wasm = exported_module!(wasm::commands);
                let wasm_proposal = exported_module!(wasm_proposal::commands);

                engine.register_static_module("wasm", wasm.into());
                engine.register_static_module("wasm::proposal", wasm_proposal.into());

                let script_path = root
                    .join(config.tasks_path)
                    .join(format!("{}.rhai", script));

                let moved_script = script.to_owned();
                let args = args.to_owned();

                // function that get global config
                engine.register_fn("get_global_config", move || {
                    to_dynamic(global_config.clone())
                });

                // function that converts string to path buf
                engine.register_fn("path", |path_str: &str| PathBuf::from(path_str));

                // function that get project root
                engine.register_fn("project_root", move || root.clone());

                // function that merges two maps
                engine.register_fn("merge", |a: Map, b: Map| {
                    let mut merged = a;
                    merged.extend(b);
                    merged
                });

                // function that matches cli args and feed into rhai script as map
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

                let script_content = read_file(script_path.clone()).map_err(|e| {
                    anyhow::anyhow!(
                        "Failed to read script `{}` ({}):\n{}",
                        script,
                        script_path.display(),
                        e.to_string()
                    )
                })?;

                let script_content = expand_macro_assert(&script_content);

                engine.run(&script_content).map_err(|e| {
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

fn expand_macro_assert(input: &str) -> String {
    let re = Regex::new(r"@assert\((?P<left>[^=!<>]+)(?P<op>==|!=|<=|<|>=|>)(?P<right>[^;]+)\);")
        .unwrap();
    let output = re.replace_all(input, |caps: &regex::Captures| {
        let left = caps.name("left").unwrap().as_str().trim();
        let right = caps.name("right").unwrap().as_str().trim();
        let op = caps.name("op").unwrap().as_str().trim();

        let escaped_left = left.replace('"', r#"\""#);
        let escaped_right = right.replace('"', r#"\""#);
        format!(
            r#"if (!({} {} {})) {{ throw "[ASSERTION FAILED]\n\n  expected:\n    {} {} {}\n\n  but:\n    left  = " + {}.to_string() + "\n    right = " + {}.to_string() + "\n\n"; }}"#,
            left, op, right, escaped_left, op, escaped_right, left, right
        )
    });

    output.into_owned()
}

fn read_file(path: impl AsRef<Path>) -> Result<String, EvalAltResult> {
    let path = path.as_ref();

    let mut f = File::open(path).map_err(|err| {
        EvalAltResult::ErrorSystem(
            format!("Cannot open script file '{}'", path.to_string_lossy()),
            err.into(),
        )
    })?;

    let mut contents = String::new();

    f.read_to_string(&mut contents).map_err(|err| {
        EvalAltResult::ErrorSystem(
            format!("Cannot read script file '{}'", path.to_string_lossy()),
            err.into(),
        )
    })?;

    if contents.starts_with("#!") {
        // Remove shebang
        if let Some(n) = contents.find('\n') {
            contents.drain(0..n).count();
        } else {
            contents.clear();
        }
    };

    Ok(contents)
}
