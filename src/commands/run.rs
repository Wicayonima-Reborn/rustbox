use anyhow::{Result, bail};
use std::env;

use crate::toolchain::{layout::ToolchainLayout, detect, env::EnvBuilder};
use crate::utils::process::run_command;

pub fn run() -> Result<()> {
    let cwd = env::current_dir()?;
    let layout = ToolchainLayout::discover(cwd);

    detect::validate(&layout)?;

    let env_builder = EnvBuilder::new(&layout);
    let status = run_command("cargo", &["run"], env_builder.as_map())?;

    if !status.success() {
        bail!("cargo run failed");
    }

    Ok(())
}