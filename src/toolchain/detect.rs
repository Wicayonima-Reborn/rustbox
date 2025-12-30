use std::path::Path;
use crate::toolchain::layout::ToolchainLayout;
use crate::error::RustBoxError;

pub fn validate(layout: &ToolchainLayout) -> Result<(), RustBoxError> {
    let required_dirs = [
        &layout.rust_bin,
        &layout.msvc_bin,
        &layout.sdk_include,
        &layout.sdk_lib,
    ];

    for dir in required_dirs {
        if !Path::new(dir).exists() {
            return Err(RustBoxError::MissingToolchainDir(dir.clone()));
        }
    }

    Ok(())
}