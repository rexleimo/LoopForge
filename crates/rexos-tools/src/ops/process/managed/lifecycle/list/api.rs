use crate::Toolset;

impl Toolset {
    pub(crate) async fn process_list(&self) -> anyhow::Result<String> {
        let entries = super::state::cloned_process_entries(&self.processes).await;

        let mut out = Vec::new();
        for (process_id, entry) in entries {
            let mut guard = entry.lock().await;
            super::state::refresh_exit_code(&mut guard).await?;
            out.push(super::summary::process_summary(
                &process_id,
                &guard.command,
                &guard.args,
                guard.exit_code,
                guard.started_at.elapsed().as_secs(),
            ));
        }

        Ok(serde_json::Value::Array(out).to_string())
    }
}
