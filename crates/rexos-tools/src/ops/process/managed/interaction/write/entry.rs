use std::sync::Arc;

use crate::process_runtime::{ProcessEntry, ProcessManager};

pub(super) async fn find_process_entry(
    processes: &tokio::sync::Mutex<ProcessManager>,
    process_id: &str,
) -> anyhow::Result<Arc<tokio::sync::Mutex<ProcessEntry>>> {
    let manager = processes.lock().await;
    manager
        .processes
        .get(process_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("unknown process_id: {process_id}"))
}
