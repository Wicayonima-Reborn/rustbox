use std::path::Path;
use std::process::Command;

use crate::toolchain::layout::ToolchainLayout;
use crate::error::RustBoxError;

// Validate required toolchain directories
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

// Validate cargo availability
pub fn validate_cargo() -> Result<(), RustBoxError> {
    let status = Command::new("cargo")
        .arg("--version")
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err(RustBoxError::MissingTool("cargo")),
    }
}

// Validate rustc availability
pub fn validate_rustc() -> Result<(), RustBoxError> {
    let status = Command::new("rustc")
        .arg("--version")
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err(RustBoxError::MissingTool("rustc")),
    }
}

// Validate MSVC minimal binaries (file existence only)
pub fn validate_msvc_binaries(layout: &ToolchainLayout)
    -> Result<(), RustBoxError>
{
    let required_bins = ["cl.exe", "link.exe", "lib.exe"];

    for bin in required_bins {
        let path = layout.msvc_bin.join(bin);
        if !path.exists() {
            return Err(RustBoxError::MissingMsvcBinary(bin));
        }
    }

    Ok(())
}