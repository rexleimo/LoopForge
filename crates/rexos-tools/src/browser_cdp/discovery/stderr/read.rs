use std::time::Duration;

use crate::browser_cdp::CDP_CONNECT_TIMEOUT_SECS;
use tokio::io::BufReader;
use tokio::io::Lines;
use tokio::process::ChildStderr;

pub(super) fn stderr_deadline() -> tokio::time::Instant {
    tokio::time::Instant::now() + Duration::from_secs(CDP_CONNECT_TIMEOUT_SECS)
}

pub(super) async fn next_stderr_line(
    deadline: tokio::time::Instant,
    lines: &mut Lines<BufReader<ChildStderr>>,
) -> Result<std::io::Result<Option<String>>, tokio::time::error::Elapsed> {
    tokio::time::timeout_at(deadline, lines.next_line()).await
}
