use crate::Toolset;

impl Toolset {
    pub(crate) async fn process_kill(&self, process_id: &str) -> anyhow::Result<String> {
        let entry = {
            let mut mgr = self.processes.lock().await;
            mgr.processes
                .remove(process_id)
                .ok_or_else(|| anyhow::anyhow!("unknown process_id: {process_id}"))?
        };

        let mut guard = entry.lock().await;
        let _ = guard.child.kill().await;
        let _ = guard.child.wait().await;

        Ok(r#"{"status":"killed"}"#.to_string())
    }
}
