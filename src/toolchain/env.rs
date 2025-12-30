use std::collections::HashMap;
use std::env;
use crate::toolchain::layout::ToolchainLayout;

pub struct EnvBuilder {
    vars: HashMap<String, String>,
}

impl EnvBuilder {
    pub fn new(layout: &ToolchainLayout) -> Self {
        let mut vars = HashMap::new();

        // Base PATH (inherit from parent)
        let base_path = env::var("PATH").unwrap_or_default();

        let path = format!(
            "{};{};{}",
            layout.rust_bin.display(),
            layout.msvc_bin.display(),
            base_path
        );

        vars.insert("PATH".into(), path);

        // INCLUDE
        let include = format!(
            "{};{}",
            layout.sdk_include.display(),
            layout.root.join("toolchain").join("msvc").join("include").display()
        );

        vars.insert("INCLUDE".into(), include);

        // LIB
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