use std::process::Command;

use anyhow::Context;

pub fn run_command(program: &str, args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new(program)
        .args(args)
        .status()
        .with_context(|| format!("Failed to run `{}`", program))?;

    if !status.success() {
        anyhow::bail!(
            "`{} {}` exited with status {}",
            program,
            args.join(" "),
            status
        );
    }
    Ok(())
}
