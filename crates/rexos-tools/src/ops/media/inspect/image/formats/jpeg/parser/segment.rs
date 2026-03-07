pub(super) fn segment_bounds(bytes: &[u8], index: &mut usize) -> Option<(usize, usize)> {
    let segment_length = super::super::scan::jpeg_segment_length(bytes.get(*index..*index + 2)?)?;
    *index += 2;

    if segment_length < 2 {
        return None;
    }

    let segment_start = *index;
    let segment_end = segment_start + segment_length - 2;
    if segment_end > bytes.len() {
        return None;
    }

    *index = segment_end;
    Some((segment_start, segment_end))
}
