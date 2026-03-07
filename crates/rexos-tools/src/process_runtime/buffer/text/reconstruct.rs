use super::super::ProcessOutputBuffer;

pub(super) fn reconstruct_all_bytes(buffer: &ProcessOutputBuffer) -> Vec<u8> {
    if buffer.total_bytes <= buffer.tail.len() {
        return buffer.tail.clone();
    }

    let tail_start = buffer.total_bytes.saturating_sub(buffer.tail.len());
    let overlap = buffer.head.len().saturating_sub(tail_start);

    let mut out = Vec::with_capacity(buffer.head.len() + buffer.tail.len().saturating_sub(overlap));
    out.extend_from_slice(&buffer.head);
    if overlap < buffer.tail.len() {
        out.extend_from_slice(&buffer.tail[overlap..]);
    }
    out
}
