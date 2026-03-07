use std::sync::Arc;

pub(crate) struct ProcessEntry {
    pub(crate) command: String,
    pub(crate) args: Vec<String>,
    pub(crate) started_at: std::time::Instant,
    pub(crate) exit_code: Option<i32>,
    pub(crate) child: tokio::process::Child,
    pub(crate) stdin: Option<tokio::process::ChildStdin>,
    pub(crate) stdout: Arc<tokio::sync::Mutex<super::ProcessOutputBuffer>>,
    pub(crate) stderr: Arc<tokio::sync::Mutex<super::ProcessOutputBuffer>>,
}

impl Drop for ProcessEntry {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}
