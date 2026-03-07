pub(super) fn jpeg_segment_length(bytes: &[u8]) -> Option<usize> {
    let len = bytes.get(0..2)?;
    Some(u16::from_be_bytes([len[0], len[1]]) as usize)
}

pub(super) fn sof_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 5 {
        return None;
    }

    let height = u16::from_be_bytes([bytes[1], bytes[2]]) as u32;
    let width = u16::from_be_bytes([bytes[3], bytes[4]]) as u32;
    Some((width, height))
}
