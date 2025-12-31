use clap::{Parser, Subcommand};
use anyhow::Result;

use crate::commands;

#[derive(Parser)]
#[command(
    name = "rustbox",
    about = "Portable MSVC-compatible Rust environment for Windows",
    disable_version_flag = true
)]
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

    /// Forward all arguments to Cargo
    Cargo {
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true
        )]
        args: Vec<String>,
    },

    /// Alias for `cargo build`
    Build {
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true
        )]
        args: Vec<String>,
    },

    /// Alias for `cargo run`
    Run {
        #[arg(
            trailing_var_arg = true,
            allow_hyphen_values = true
        )]
        args: Vec<String>,
    },
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    pub fn execute(self) -> Result<()> {
        match self.command {
            Commands::Init => commands::init::run(),

            Commands::New { name } => commands::new::run(&name),

            Commands::Cargo { args } => {
                commands::cargo::run(args)
            }

            Commands::Build { args } => {
                let mut full_args = vec!["build".to_string()];
                full_args.extend(args);
                commands::cargo::run(full_args)
            }

            Commands::Run { args } => {
                let mut full_args = vec!["run".to_string()];
                full_args.extend(args);
                commands::cargo::run(full_args)
            }
        }
    }
}