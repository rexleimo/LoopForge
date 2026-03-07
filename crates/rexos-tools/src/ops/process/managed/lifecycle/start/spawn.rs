use std::process::Stdio;

use anyhow::Context;

pub(super) fn spawn_process(
    command: &str,
    args: &[String],
    workspace_root: &std::path::Path,
) -> anyhow::Result<tokio::process::Child> {
    let mut cmd = tokio::process::Command::new(command);
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    crate::ops::process::configure_command_env(&mut cmd, workspace_root);
    cmd.spawn().context("spawn process")
}
