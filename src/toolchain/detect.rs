use std::path::Path;
use std::fs;

use crate::toolchain::layout::ToolchainLayout;
use crate::error::RustBoxError;

/// Verifies the presence of critical toolchain directory structures.
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

/// Validates the existence of essential MSVC build-tool binaries.
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

/// Validates the portable Rust toolchain and dynamically verifies the target standard library.
pub fn validate_rust_toolchain(
    layout: &ToolchainLayout,
) -> Result<(), RustBoxError> {
    // 1. Verify primary executable binaries
    let cargo = layout.rust_bin.join("cargo.exe");
    let rustc = layout.rust_bin.join("rustc.exe");

    if !cargo.exists() {
        return Err(RustBoxError::MissingTool("cargo.exe"));
    }

    if !rustc.exists() {
        return Err(RustBoxError::MissingTool("rustc.exe"));
    }

    // 2. Resolve the rustlib metadata directory
    let rustlib_root = layout
        .root
        .join("toolchain")
        .join("rust")
        .join("lib")
        .join("rustlib");

    if !rustlib_root.exists() {
        return Err(RustBoxError::MissingToolchainDir(rustlib_root));
    }

    // 3. Perform dynamic lookup for the MSVC target triple directory
    // This ensures compatibility even if the exact folder name varies slightly
    let mut target_exists = false;
    let entries = fs::read_dir(&rustlib_root).map_err(|_| {
        RustBoxError::MissingToolchainDir(rustlib_root.clone())
    })?;

    for entry in entries {
        if let Ok(entry) = entry {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if name_str.contains("x86_64-pc-windows-msvc") {
                target_exists = true;
                break;
            }
        }
    }

    if !target_exists {
        return Err(RustBoxError::MissingToolchainDir(
            rustlib_root.join("x86_64-pc-windows-msvc"),
        ));
    }

    Ok(())
}