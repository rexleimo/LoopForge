use anyhow::bail;

pub(super) fn validate_shell_command(command: &str) -> anyhow::Result<()> {
    if command.trim().is_empty() {
        bail!("command is empty");
    }

    if command.contains("rm -rf /") || command.contains("sudo ") {
        bail!("command denied by policy");
    }

    Ok(())
}
