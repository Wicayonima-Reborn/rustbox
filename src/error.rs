use thiserror::Error;
use std::path::PathBuf;

#[derive(Debug, Error)]
pub enum RustBoxError {
    #[error("Missing required toolchain directory: {0}")]
    MissingToolchainDir(PathBuf),

    #[error("Required tool not found in PATH: {0}")]
    MissingTool(&'static str),

    #[error("Missing MSVC binary: {0}")]
    MissingMsvcBinary(&'static str),
}