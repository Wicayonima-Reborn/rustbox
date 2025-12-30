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
    /// - Future distribution mode: bin/rustbox.exe
    pub fn root_from_exe() -> PathBuf {
        let exe = std::env::current_exe()
            .expect("cannot determine current executable path");

        let mut dir = exe
            .parent()
            .expect("executable must have a parent directory")
            .to_path_buf();

        // Development mode: target/debug or target/release
        if dir.ends_with("debug") || dir.ends_with("release") {
            // target/
            if let Some(target_dir) = dir.parent() {
                // project root
                if let Some(project_root) = target_dir.parent() {
                    dir = project_root.to_path_buf();
                }
            }
        }

        dir
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