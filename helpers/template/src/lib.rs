use anyhow::Context;
use anyhow::Result;
use cargo_generate::{generate as cargo_generate, Cli as CargoGen};
use std::{env, fs, path::Path};
use structopt::StructOpt;

pub struct Template<'a> {
    /// name of the generated directory
    name: &'a str,
    /// git repo to be used as a template
    repo: &'a str,
    /// git branch to be used as a template
    branch: &'a str,
    /// target directory as base for code generation
    target_dir: &'a Path,
}

impl Template<'_> {
    pub fn new<'a>(
        name: &'a str,
        repo: &'a str,
        branch: &'a str,
        target_dir: &'a Path,
    ) -> Template<'a> {
        Template {
            repo,
            name,
            branch,
            target_dir,
        }
    }
    pub fn generate(self: &Self) -> Result<()> {
        let target_dir_display = self.target_dir.display();
        let current_dir = env::current_dir().with_context(|| "Unable to get current directory.")?;
        fs::create_dir_all(self.target_dir)
            .with_context(|| format!("Unable to create directory: {target_dir_display}"))?;
        env::set_current_dir(Path::new(self.target_dir)).with_context(|| {
            format!("Unable to set current directory to {target_dir_display}`.")
        })?;

        let CargoGen::Generate(args) = CargoGen::from_iter(
            vec![
                "cargo",
                "generate",
                &self.repo,
                "--branch",
                &self.branch,
                "--name",
                &self.name,
            ]
            .iter(),
        );

        let name = self.name;
        let repo = self.repo;
        let branch = self.branch;

        cargo_generate(args)
        .with_context(|| format!("Unable to generate contract `{name}` with template `{repo}:{branch}` to `{target_dir_display}`."))?;

        env::set_current_dir(current_dir.as_path()).with_context(|| {
            format!("Unable to set current directory back to current directory after changed to `{target_dir_display}`.")
        })
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
