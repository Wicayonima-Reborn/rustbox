mod cli;
mod commands;
mod toolchain;
mod config;
mod utils;
mod error;

use anyhow::Result;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.execute()
}