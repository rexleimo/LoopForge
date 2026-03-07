mod decode;
mod reconstruct;
mod state;
#[cfg(test)]
mod tests;

use super::ProcessOutputBuffer;

pub(super) fn take_text(buffer: &mut ProcessOutputBuffer) -> (String, bool) {
    if buffer.total_bytes == 0 {
        return (String::new(), false);
    }

    let truncated = state::is_truncated(buffer.total_bytes);
    let output = if truncated {
        decode::decode_truncated_text(&buffer.head, &buffer.tail)
    } else {
        let bytes = reconstruct_all_bytes(buffer);
        decode::decode_full_text(bytes)
    };

    state::clear_buffer(buffer);
    (output, truncated)
}

pub(super) fn reconstruct_all_bytes(buffer: &ProcessOutputBuffer) -> Vec<u8> {
    reconstruct::reconstruct_all_bytes(buffer)
}
