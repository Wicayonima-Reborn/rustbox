use std::path::PathBuf;

#[derive(Debug)]
pub struct ToolchainLayout {
    pub root: PathBuf,
    pub rust_bin: PathBuf,
    pub msvc_bin: PathBuf,
    pub sdk_include: PathBuf,
    pub sdk_lib: PathBuf,
}

impl ToolchainLayout {
    /// Root directory resolved from the running executable
    pub fn root_from_exe() -> PathBuf {
        let exe = std::env::current_exe()
            .expect("cannot determine current executable path");

        exe.parent()
            .expect("executable must have a parent directory")
            .to_path_buf()
    }

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