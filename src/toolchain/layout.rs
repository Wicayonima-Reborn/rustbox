use std::path::{PathBuf};

#[derive(Debug)]
pub struct ToolchainLayout {
    pub root: PathBuf,
    pub rust_bin: PathBuf,
    pub msvc_bin: PathBuf,
    pub sdk_include: PathBuf,
    pub sdk_lib: PathBuf,
}

impl ToolchainLayout {
    pub fn discover(root: PathBuf) -> Self {
        let toolchain = root.join("toolchain");

        Self {
            root,
            rust_bin: toolchain.join("rust").join("bin"),
            msvc_bin: toolchain.join("msvc").join("bin"),
            sdk_include: toolchain.join("windows-sdk").join("include"),
            sdk_lib: toolchain.join("windows-sdk").join("lib"),
        }
    }
}