use super::super::ProcessOutputBuffer;

pub(super) fn is_truncated(total_bytes: usize) -> bool {
    total_bytes > super::super::super::PROCESS_OUTPUT_MAX_BYTES
}

pub(super) fn clear_buffer(buffer: &mut ProcessOutputBuffer) {
    buffer.head.clear();
    buffer.tail.clear();
    buffer.total_bytes = 0;
}
