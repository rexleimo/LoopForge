use crate::defs::{DockerExecArgs, ShellArgs, ShellExecArgs};
use crate::Toolset;

pub(super) async fn dispatch(
    toolset: &Toolset,
    name: &str,
    arguments_json: &str,
) -> anyhow::Result<String> {
    match name {
        "shell" => {
            let args: ShellArgs = super::super::parse_args(arguments_json, name)?;
            toolset.shell(&args.command, args.timeout_ms).await
        }
        "shell_exec" => {
            let args: ShellExecArgs = super::super::parse_args(arguments_json, name)?;
            let timeout_ms = args
                .timeout_seconds
                .map(|seconds| seconds.saturating_mul(1000));
            toolset.shell(&args.command, timeout_ms).await
        }
        "docker_exec" => {
            let args: DockerExecArgs = super::super::parse_args(arguments_json, name)?;
            toolset.docker_exec(&args.command).await
        }
        _ => unreachable!("unexpected exec process tool: {name}"),
    }
}
