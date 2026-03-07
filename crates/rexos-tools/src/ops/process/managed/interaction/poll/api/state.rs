use std::sync::Arc;

use anyhow::Context;

use crate::process_runtime::{ProcessEntry, ProcessOutputBuffer};

pub(super) struct ProcessPollState {
    pub(super) stdout: Arc<tokio::sync::Mutex<ProcessOutputBuffer>>,
    pub(super) stderr: Arc<tokio::sync::Mutex<ProcessOutputBuffer>>,
    pub(super) exit_code: Option<i32>,
    pub(super) alive: bool,
}

pub(super) async fn poll_process_state(
    entry: &Arc<tokio::sync::Mutex<ProcessEntry>>,
) -> anyhow::Result<ProcessPollState> {
    let mut guard = entry.lock().await;
    if guard.exit_code.is_none() {
        if let Some(status) = guard.child.try_wait().context("try_wait process")? {
            guard.exit_code = Some(status.code().unwrap_or(-1));
        }
    }

    Ok(ProcessPollState {
        stdout: guard.stdout.clone(),
        stderr: guard.stderr.clone(),
        exit_code: guard.exit_code,
        alive: super::super::status::alive_from_exit_code(guard.exit_code),
    })
}
