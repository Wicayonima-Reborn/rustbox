use std::path::Path;

use crate::toolchain::layout::ToolchainLayout;
use crate::error::RustBoxError;

/// Validate required toolchain directories
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

/// Validate MSVC minimal binaries (file existence only)
pub fn validate_msvc_binaries(
    layout: &ToolchainLayout,
) -> Result<(), RustBoxError> {
    let required_bins = ["cl.exe", "link.exe", "lib.exe"];

    for bin in required_bins {
        let path = layout.msvc_bin.join(bin);
        if !path.exists() {
            return Err(RustBoxError::MissingMsvcBinary(bin));
        }
    }

    Ok(())
}

/// Validate bundled Rust toolchain (portable)
pub fn validate_rust_toolchain(
    layout: &ToolchainLayout,
) -> Result<(), RustBoxError> {
    let cargo = layout.rust_bin.join("cargo.exe");
    let rustc = layout.rust_bin.join("rustc.exe");

    if !cargo.exists() {
        return Err(RustBoxError::MissingTool("cargo.exe"));
    }

    if !rustc.exists() {
        return Err(RustBoxError::MissingTool("rustc.exe"));
    }

    let rustlib_target = layout
        .root
        .join("toolchain")
        .join("rust")
        .join("lib")
        .join("rustlib")
        .join("x86_64-pc-windows-msvc");

    if !rustlib_target.exists() {
        return Err(RustBoxError::MissingToolchainDir(rustlib_target));
    }

    Ok(())
}