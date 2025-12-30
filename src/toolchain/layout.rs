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
    /// Resolve RustBox root directory from the running executable.
    ///
    /// Handles:
    /// - Development mode: target/debug or target/release
    /// - Distribution mode: bin/rustbox(.exe)
    pub fn root_from_exe() -> PathBuf {
        let exe = std::env::current_exe()
            .expect("cannot determine current executable path");

        let dir = exe
            .parent()
            .expect("executable must have a parent directory");

        // Case 1: distribution layout -> bin/rustbox(.exe)
        // root = parent of bin
        if dir.file_name().map(|n| n == "bin").unwrap_or(false) {
            return dir
                .parent()
                .expect("bin directory must have a parent")
                .to_path_buf();
        }

        // Case 2: development layout -> target/debug or target/release
        if dir.file_name().map(|n| n == "debug" || n == "release").unwrap_or(false) {
            // target/
            let target_dir = dir
                .parent()
                .expect("debug/release must have a parent");

            // project root
            return target_dir
                .parent()
                .expect("target directory must have a parent")
                .to_path_buf();
        }

        // Fallback: assume exe directory is root
        dir.to_path_buf()
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