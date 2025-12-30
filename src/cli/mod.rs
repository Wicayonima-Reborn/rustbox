use clap::{Parser, Subcommand};
use anyhow::Result;

use crate::commands;

#[derive(Parser)]
#[command(name = "rustbox")]
#[command(about = "Portable MSVC-compatible Rust environment for Windows")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    New {
        name: String,
    },
    Build,
    Run,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::Init => commands::init::run(),
            Commands::New { name } => commands::new::run(&name),
            Commands::Build => commands::build::run(),
            Commands::Run => commands::run::run(),
        }
    }
}