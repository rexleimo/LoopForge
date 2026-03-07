use std::sync::Arc;

use anyhow::Context;

use crate::Toolset;

impl Toolset {
    pub(crate) async fn process_start(
        &self,
        command: &str,
        args: &[String],
    ) -> anyhow::Result<String> {
        super::validate::validate_process_start(command)?;

        let mut mgr = self.processes.lock().await;
        super::validate::enforce_process_limit(mgr.processes.len())?;

        let process_id = uuid::Uuid::new_v4().to_string();
        let mut child = super::spawn::spawn_process(command, args, &self.workspace_root)?;
        let stdin = child.stdin.take();
        let stdout = child.stdout.take().context("process stdout is not piped")?;
        let stderr = child.stderr.take().context("process stderr is not piped")?;

        let (stdout_buf, stderr_buf) = super::buffers::process_output_buffers();
        Self::spawn_process_output_reader(stdout, stdout_buf.clone());
        Self::spawn_process_output_reader(stderr, stderr_buf.clone());

        let entry =
            super::entry::process_entry(command, args, child, stdin, stdout_buf, stderr_buf);

        mgr.processes
            .insert(process_id.clone(), Arc::new(tokio::sync::Mutex::new(entry)));

        Ok(super::response::process_start_payload(&process_id).to_string())
    }
}
