use anyhow::{Result, Context};
use std::process::Command;

pub fn run(args: Vec<String>) -> Result<()> {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .context("failed to execute cargo")?;

    if !status.success() {
        anyhow::bail!("cargo exited with status {}", status);
    }

    Ok(())
}