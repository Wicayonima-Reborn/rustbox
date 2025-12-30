use anyhow::{Result, Context, bail};
use crate::toolchain::{layout::ToolchainLayout, detect, env::EnvBuilder};
use crate::utils::process::run_command;

/// Initializes a new Rust project using the internal RustBox toolchain.
pub fn run(project_name: &str) -> Result<()> {
    // Resolve the RustBox root directory and toolchain filesystem layout
    let root = ToolchainLayout::root_from_exe();
    let layout = ToolchainLayout::discover(root);

    // Perform pre-runtime validation of toolchain components and dependencies
    detect::validate(&layout).context("Failed initial toolchain validation")?;
    detect::validate_msvc_binaries(&layout).context("MSVC build tools not found or invalid")?;
    detect::validate_rust_toolchain(&layout).context("Rust toolchain integrity check failed")?;

    // Initialize the environment variable builder based on the resolved layout
    let env_builder = EnvBuilder::new(&layout);

    // Construct the absolute path to the bundled Cargo binary
    let cargo_path = layout
        .rust_bin
        .join("cargo.exe")
        .to_string_lossy()
        .to_string();

    // Execute 'cargo new <name>' within the isolated toolchain environment
    let status = run_command(
        &cargo_path, 
        &["new", project_name], 
        env_builder.as_map()
    ).context("Failed to execute cargo subprocess")?;

    if !status.success() {
        bail!("Cargo process exited with non-zero status during project initialization");
    }

    Ok(())
}