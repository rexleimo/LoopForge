pub(super) fn parse_gif_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 10 {
        return None;
    }
    if bytes.get(0..6)? != b"GIF87a" && bytes.get(0..6)? != b"GIF89a" {
        return None;
    }

    let width = u16::from_le_bytes(bytes.get(6..8)?.try_into().ok()?) as u32;
    let height = u16::from_le_bytes(bytes.get(8..10)?.try_into().ok()?) as u32;
    Some((width, height))
}
