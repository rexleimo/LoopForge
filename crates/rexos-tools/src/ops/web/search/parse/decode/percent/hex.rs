pub(super) fn decoded_byte(hi: u8, lo: u8) -> Option<u8> {
    Some(hex_value(hi)? * 16 + hex_value(lo)?)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
