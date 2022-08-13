use anyhow::bail;

pub fn run_command(cmd: &mut std::process::Command) -> Result<(), anyhow::Error> {
    let exit_status = cmd.spawn()?.wait()?;
    if !exit_status.success() {
        bail!("Failed to execute: `{:#?}`", cmd)
    }
    Ok(())
}
