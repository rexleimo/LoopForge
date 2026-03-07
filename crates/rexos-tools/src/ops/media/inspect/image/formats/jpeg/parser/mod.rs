mod cursor;
mod segment;
#[cfg(test)]
mod tests;

pub(super) fn parse_jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return None;
    }

    let mut index = 2usize;
    while let Some(marker) = cursor::next_marker(bytes, &mut index) {
        if super::markers::is_terminal_marker(marker) {
            break;
        }

        let Some((segment_start, segment_end)) = segment::segment_bounds(bytes, &mut index) else {
            break;
        };

        if super::markers::is_jpeg_sof(marker) {
            return super::scan::sof_dimensions(&bytes[segment_start..segment_end]);
        }
    }

    None
}
