use std::collections::HashMap;
use std::env;

use crate::toolchain::layout::ToolchainLayout;

pub struct EnvBuilder {
    vars: HashMap<String, String>,
}

impl EnvBuilder {
    pub fn new(layout: &ToolchainLayout) -> Self {
        let mut vars = HashMap::new();

        // Redirect Cargo home to ensure toolchain isolation and portability
        let cargo_home = layout.root.join("cargo-home");
        vars.insert(
            "CARGO_HOME".into(),
            cargo_home.to_string_lossy().into_owned(),
        );

        // Prepend toolchain and MSVC binaries to PATH to override system defaults
        let base_path = env::var("PATH").unwrap_or_default();
        let path = format!(
            "{};{};{};{}",
            layout.rust_bin.display(),
            layout.msvc_bin.display(),
            cargo_home.join("bin").display(),
            base_path
        );
        vars.insert("PATH".into(), path);

        // Set MSVC include directories for C++ header resolution
        let include = format!(
            "{};{}",
            layout.sdk_include.display(),
            layout.root.join("toolchain").join("msvc").join("include").display()
        );
        vars.insert("INCLUDE".into(), include);

        // Set MSVC library directories for the linker
        let lib = format!(
            "{};{}",
            layout.sdk_lib.display(),
            layout.root.join("toolchain").join("msvc").join("lib").display()
        );
        vars.insert("LIB".into(), lib);

        Self { vars }
    }

    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.vars
    }
}