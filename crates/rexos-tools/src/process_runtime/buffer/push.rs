use super::ProcessOutputBuffer;

pub(super) fn push_chunk(buffer: &mut ProcessOutputBuffer, chunk: &[u8]) {
    if chunk.is_empty() {
        return;
    }

    buffer.total_bytes = buffer.total_bytes.saturating_add(chunk.len());

    if buffer.head.len() < super::super::PROCESS_OUTPUT_HEAD_MAX_BYTES {
        let remaining = super::super::PROCESS_OUTPUT_HEAD_MAX_BYTES - buffer.head.len();
        let take = remaining.min(chunk.len());
        buffer.head.extend_from_slice(&chunk[..take]);
    }

    buffer.tail.extend_from_slice(chunk);
    if buffer.tail.len() > super::super::PROCESS_OUTPUT_TAIL_MAX_BYTES {
        let start = buffer.tail.len() - super::super::PROCESS_OUTPUT_TAIL_MAX_BYTES;
        let tail = buffer.tail.split_off(start);
        buffer.tail = tail;
    }
}
