use anyhow::Context;
use anyhow::Result;
use cargo_generate::{generate as cargo_generate, Cli as CargoGen};
use clap::Parser;
use derive_get_docs::GetDocs;
use derive_new::new;
use getset::Getters;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Clone, Deserialize, Serialize, Getters, new, GetDocs)]
#[get = "pub"]
pub struct Template {
    /// Name of the generated directory
    name: String,
    /// Git repo to be used as a template
    repo: String,
    branch: String,
    target_dir: PathBuf,
    subfolder: Option<String>,
}

impl Template {
    pub fn with_name(&self, name: Option<String>) -> Template {
        Template {
            name: name.unwrap_or_else(|| self.name.clone()),
            ..self.clone()
        }
    }
    pub fn with_branch(&self, branch: Option<String>) -> Template {
        Template {
            branch: branch.unwrap_or_else(|| self.branch.clone()),
            ..self.clone()
        }
    }
    pub fn with_target_dir(&self, target_dir: Option<PathBuf>) -> Template {
        Template {
            target_dir: target_dir.unwrap_or_else(|| self.target_dir.clone()),
            ..self.clone()
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
            [argv, vec!["--", subfolder]].concat()
        } else {
            argv
        };

        let CargoGen::Generate(args) = CargoGen::parse_from(argv.iter());

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
