mod entry;
mod output;
mod state;
#[cfg(test)]
mod tests;

use crate::Toolset;

impl Toolset {
    pub(crate) async fn process_poll(&self, process_id: &str) -> anyhow::Result<String> {
        let entry = entry::find_process_entry(&self.processes, process_id).await?;
        let snapshot = state::poll_process_state(&entry).await?;
        let (stdout, stdout_truncated) = output::take_buffer_text(&snapshot.stdout).await;
        let (stderr, stderr_truncated) = output::take_buffer_text(&snapshot.stderr).await;

        Ok(super::response::process_poll_payload(
            &stdout,
            &stderr,
            stdout_truncated,
            stderr_truncated,
            snapshot.exit_code,
            snapshot.alive,
        )
        .to_string())
    }
}
