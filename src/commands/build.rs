use anyhow::{Result, bail};

use crate::toolchain::{layout::ToolchainLayout, detect, env::EnvBuilder};
use crate::utils::process::run_command;

pub fn run() -> Result<()> {
    let root = ToolchainLayout::root_from_exe();
    let layout = ToolchainLayout::discover(root);

    detect::validate(&layout)?;
    detect::validate_msvc_binaries(&layout)?;
    detect::validate_cargo()?;
    detect::validate_rustc()?;

    let env_builder = EnvBuilder::new(&layout);
    let status = run_command("cargo", &["build"], env_builder.as_map())?;

    if !status.success() {
        bail!("cargo build failed");
    }

    Ok(())
}