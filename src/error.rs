use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum RustBoxError {
    #[error("Missing required toolchain directory: {0}")]
    MissingToolchainDir(PathBuf),
}