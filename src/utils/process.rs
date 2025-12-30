use std::collections::HashMap;
use std::process::{Command, ExitStatus};

pub fn run_command(
    program: &str,
    args: &[&str],
    envs: &HashMap<String, String>,
) -> std::io::Result<ExitStatus> {
    let mut cmd = Command::new(program);

    cmd.args(args)
        .envs(envs)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    cmd.status()
}