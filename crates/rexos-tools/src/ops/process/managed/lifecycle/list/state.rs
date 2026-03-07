use std::sync::Arc;

use anyhow::Context;

use crate::process_runtime::{ProcessEntry, ProcessManager};

pub(super) async fn cloned_process_entries(
    processes: &tokio::sync::Mutex<ProcessManager>,
) -> Vec<(String, Arc<tokio::sync::Mutex<ProcessEntry>>)> {
    let manager = processes.lock().await;
    manager
        .processes
        .iter()
        .map(|(id, entry)| (id.clone(), entry.clone()))
        .collect()
}

pub(super) async fn refresh_exit_code(guard: &mut ProcessEntry) -> anyhow::Result<()> {
    if guard.exit_code.is_none() {
        if let Some(status) = guard.child.try_wait().context("try_wait process")? {
            guard.exit_code = Some(status.code().unwrap_or(-1));
        }
    }
    Ok(())
}
