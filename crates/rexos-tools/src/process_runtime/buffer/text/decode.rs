use crate::Toolset;

pub(super) fn decode_truncated_text(head: &[u8], tail: &[u8]) -> String {
    let head = Toolset::decode_process_output(head.to_vec());
    let tail = Toolset::decode_process_output(tail.to_vec());
    format!(
        "{head}{}{tail}",
        super::super::super::TOOL_OUTPUT_MIDDLE_OMISSION_MARKER
    )
}

pub(super) fn decode_full_text(bytes: Vec<u8>) -> String {
    Toolset::decode_process_output(bytes)
}
