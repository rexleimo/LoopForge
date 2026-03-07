mod command;
mod config;

use std::time::Duration;

use anyhow::{bail, Context};

use crate::Toolset;

impl Toolset {
    pub(crate) async fn docker_exec(&self, command: &str) -> anyhow::Result<String> {
        config::ensure_docker_exec_enabled()?;
        validate_docker_command(command)?;

        let image = config::docker_exec_image();
        let timeout = Duration::from_secs(60);
        let mount = format!("{}:/workspace", self.workspace_root.display());
        let mut cmd = command::docker_exec_command(&image, &mount, command);

        let output = tokio::time::timeout(timeout, cmd.output())
            .await
            .context("docker_exec timed out")?
            .context("spawn docker")?;

        Ok(command::docker_exec_result(output, image))
    }
}

fn validate_docker_command(command: &str) -> anyhow::Result<()> {
    if command.trim().is_empty() {
        bail!("command is empty");
    }
    Ok(())
}
