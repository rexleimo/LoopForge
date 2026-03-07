use std::sync::Arc;

use crate::process_runtime::ProcessOutputBuffer;

pub(super) async fn take_buffer_text(
    buffer: &Arc<tokio::sync::Mutex<ProcessOutputBuffer>>,
) -> (String, bool) {
    let mut guard = buffer.lock().await;
    guard.take_text()
}
