use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use tokio::io::AsyncWriteExt;

use crate::process_runtime::ProcessEntry;

const PROCESS_WRITE_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) async fn write_to_process_stdin(
    entry: &Arc<tokio::sync::Mutex<ProcessEntry>>,
    data: &str,
) -> anyhow::Result<()> {
    let mut guard = entry.lock().await;
    let stdin = guard.stdin.as_mut().context("process stdin is closed")?;

    tokio::time::timeout(PROCESS_WRITE_TIMEOUT, stdin.write_all(data.as_bytes()))
        .await
        .context("process_write timed out")?
        .context("write stdin")?;
    tokio::time::timeout(PROCESS_WRITE_TIMEOUT, stdin.flush())
        .await
        .context("process_write timed out")?
        .context("flush stdin")?;

    Ok(())
}
