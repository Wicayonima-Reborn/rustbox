use anyhow::Result;
use std::env;
use crate::toolchain::{layout::ToolchainLayout, detect};

pub fn run() -> Result<()> {
    let cwd = env::current_dir()?;
    let layout = ToolchainLayout::discover(cwd);

    detect::validate(&layout)?;

    println!("RustBox environment looks OK.");
    Ok(())
}