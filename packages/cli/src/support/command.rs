use anyhow::bail;

pub fn run_commands(commands: &[&[&str]]) -> Result<(), anyhow::Error> {
    for c in commands {
        assert!(c.len() > 0);
        let prog = c[0];
        let args = if c.len() > 1 { &c[1..] } else { &[] as &[&str] };

        let exit_status = std::process::Command::new(prog).args(args).spawn()?.wait()?;
        if !exit_status.success() {
            bail!("Failed to execute: `{} {}`", prog, args.join(" "))
        }
    }

    Ok(())
}

#[macro_export]
macro_rules! run_commands {
    ($($cmd:tt),*) => {
        $crate::support::command::run_commands(&[
            $($crate::run_commands!(@single, $cmd)),*
        ])
    };
    (@single, { > $($cmd:expr),* }) => {
        &[$(($cmd)),*]
    };
}
