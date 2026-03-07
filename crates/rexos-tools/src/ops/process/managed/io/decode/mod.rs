mod detect;
mod utf16;

pub(crate) fn decode_process_output(bytes: Vec<u8>) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    if detect::likely_utf16(&bytes) {
        return utf16::decode_utf16_lossy(bytes);
    }

    String::from_utf8_lossy(&bytes).to_string()
}
