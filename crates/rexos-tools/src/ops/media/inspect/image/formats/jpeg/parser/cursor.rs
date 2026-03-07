pub(super) fn next_marker(bytes: &[u8], index: &mut usize) -> Option<u8> {
    while *index + 1 < bytes.len() {
        if bytes[*index] != 0xFF {
            *index += 1;
            continue;
        }

        while *index < bytes.len() && bytes[*index] == 0xFF {
            *index += 1;
        }
        if *index >= bytes.len() {
            break;
        }

        let marker = bytes[*index];
        *index += 1;
        return Some(marker);
    }

    None
}
