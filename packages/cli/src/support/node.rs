use std::{ffi::OsStr, process::Command};
use anyhow::bail;

pub fn run_npx<I, S>(args: I, error_context: &str) -> Result<(), anyhow::Error>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let npx = Command::new("npx").args(args).spawn();

    match npx {
        Ok(mut npx) => {
            if !npx.wait()?.success() {
                bail!("Failed to execute npx");
            }
            Ok(())
        }
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                bail!("`npx`, which pre-bundled with npm >= 5.2.0, is required for console but not found. Please install `npm` or check your path.")
            } else {
                use anyhow::Context as _;
                Err(e).with_context(|| error_context.to_string())
            }
        }
    }
}
