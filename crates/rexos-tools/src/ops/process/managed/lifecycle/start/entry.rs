use std::sync::Arc;

use crate::process_runtime::{ProcessEntry, ProcessOutputBuffer};

pub(super) fn process_entry(
    command: &str,
    args: &[String],
    child: tokio::process::Child,
    stdin: Option<tokio::process::ChildStdin>,
    stdout_buf: Arc<tokio::sync::Mutex<ProcessOutputBuffer>>,
    stderr_buf: Arc<tokio::sync::Mutex<ProcessOutputBuffer>>,
) -> ProcessEntry {
    ProcessEntry {
        command: command.to_string(),
        args: args.to_vec(),
        started_at: std::time::Instant::now(),
        exit_code: None,
        child,
        stdin,
        stdout: stdout_buf,
        stderr: stderr_buf,
    }
}
