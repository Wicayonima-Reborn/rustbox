use anyhow::Result;
use std::env;

use crate::toolchain::{
    layout::ToolchainLayout,
    detect,
    env::EnvBuilder,
};

pub fn run() -> Result<()> {
    let cwd = env::current_dir()?;
    let layout = ToolchainLayout::discover(cwd);

    detect::validate(&layout)?;

    let env_builder = EnvBuilder::new(&layout);

    println!("RustBox environment prepared:");
    for (k, v) in env_builder.as_map() {
        println!("{} = {}", k, v);
    }

    Ok(())
}