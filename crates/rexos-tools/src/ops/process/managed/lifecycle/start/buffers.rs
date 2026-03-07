use std::sync::Arc;

use crate::process_runtime::ProcessOutputBuffer;

pub(super) fn process_output_buffers() -> (
    Arc<tokio::sync::Mutex<ProcessOutputBuffer>>,
    Arc<tokio::sync::Mutex<ProcessOutputBuffer>>,
) {
    (
        Arc::new(tokio::sync::Mutex::new(ProcessOutputBuffer::default())),
        Arc::new(tokio::sync::Mutex::new(ProcessOutputBuffer::default())),
    )
}
