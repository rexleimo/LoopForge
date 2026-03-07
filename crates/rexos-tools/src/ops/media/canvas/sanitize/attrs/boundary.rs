pub(super) fn has_valid_prefix_boundary(bytes: &[u8], index: usize) -> bool {
    if index == 0 {
        return true;
    }

    let prev = bytes[index - 1];
    prev.is_ascii_whitespace() || matches!(prev, b'<' | b'"' | b'\'' | b'/' | b'=')
}

pub(super) fn skip_event_name_and_whitespace(bytes: &[u8], mut cursor: usize) -> Option<usize> {
    let mut had_letter = false;
    while cursor < bytes.len() && bytes[cursor].is_ascii_alphabetic() {
        had_letter = true;
        cursor += 1;
    }
    if !had_letter {
        return None;
    }

    while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    Some(cursor)
}
