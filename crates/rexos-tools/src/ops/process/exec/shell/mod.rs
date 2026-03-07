mod command;
mod policy;

use std::time::Duration;

use anyhow::{bail, Context};

use crate::Toolset;

impl Toolset {
    pub(crate) async fn shell(
        &self,
        command: &str,
        timeout_ms: Option<u64>,
    ) -> anyhow::Result<String> {
        policy::validate_shell_command(command)?;

        let timeout = Duration::from_millis(timeout_ms.unwrap_or(60_000));
        let mut cmd = command::shell_command(command);
        super::super::configure_command_env(&mut cmd, &self.workspace_root);

        let output = tokio::time::timeout(timeout, cmd.output())
            .await
            .context("shell timed out")?
            .context("spawn shell")?;

        let combined = command::combined_output(&output);
        if !output.status.success() {
            bail!("shell failed: {}", combined.trim());
        }

        Ok(combined)
    }
}
