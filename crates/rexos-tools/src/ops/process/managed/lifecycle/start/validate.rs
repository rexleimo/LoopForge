use anyhow::bail;

use crate::process_runtime::PROCESS_MAX_PROCESSES;

pub(super) fn validate_process_start(command: &str) -> anyhow::Result<()> {
    if command.trim().is_empty() {
        bail!("command is empty");
    }
    Ok(())
}

pub(super) fn enforce_process_limit(process_count: usize) -> anyhow::Result<()> {
    if process_count >= PROCESS_MAX_PROCESSES {
        bail!("process limit reached (max {PROCESS_MAX_PROCESSES})");
    }
    Ok(())
}
