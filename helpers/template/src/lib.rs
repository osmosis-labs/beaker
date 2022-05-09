use anyhow::Context;
use anyhow::Result;
use cargo_generate::{generate as cargo_generate, Cli as CargoGen};
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::{env, fs};
use structopt::StructOpt;

#[derive(Clone, Deserialize, Serialize, Getters)]
#[get = "pub"]
pub struct Template {
    /// name of the generated directory
    name: String,
    /// git repo to be used as a template
    repo: String,
    branch: String,
    target_dir: PathBuf,
    subfolder: Option<String>,
}

impl Template {
    pub fn new(name: String, repo: String, branch: String, target_dir: PathBuf) -> Template {
        Template {
            repo,
            name,
            branch,
            target_dir,
            subfolder: None,
        }
    }

    pub fn with_subfolder(self, subfolder: &str) -> Template {
        Template {
            subfolder: Some(subfolder.to_string()),
            ..self
        }
    }

    pub fn generate(&self) -> Result<()> {
        let target_dir_display = self.target_dir.display();
        let current_dir = env::current_dir().with_context(|| "Unable to get current directory.")?;
        fs::create_dir_all(self.target_dir.as_path())
            .with_context(|| format!("Unable to create directory: {target_dir_display}"))?;
        env::set_current_dir(self.target_dir.as_path()).with_context(|| {
            format!("Unable to set current directory to {target_dir_display}`.")
        })?;

        let argv = vec![
            "cargo",
            "generate",
            "--name",
            &self.name,
            "--git",
            &self.repo,
            "--branch",
            &self.branch,
        ];

        let argv = if let Some(subfolder) = &self.subfolder {
            [argv, vec!["--", &subfolder]].concat()
        } else {
            argv
        };

        let CargoGen::Generate(args) = CargoGen::from_iter(argv.iter());

        let name = &self.name;
        let repo = &self.repo;
        let branch = &self.branch;

        cargo_generate(args)
        .with_context(|| format!("Unable to generate contract `{name}` with template `{repo}:{branch}` to `{target_dir_display}`."))?;

        env::set_current_dir(current_dir.as_path()).with_context(|| {
            format!("Unable to set current directory back to current directory after changed to `{target_dir_display}`.")
        })
    }
}
