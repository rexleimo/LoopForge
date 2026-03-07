use super::scan::{jpeg_segment_length, sof_dimensions};

#[test]
fn jpeg_segment_length_reads_big_endian_marker_size() {
    assert_eq!(jpeg_segment_length(&[0x00, 0x11]), Some(17));
    assert_eq!(jpeg_segment_length(&[0x00]), None);
}

#[test]
fn sof_dimensions_extracts_width_and_height() {
    assert_eq!(
        sof_dimensions(&[8, 0x00, 0x10, 0x00, 0x20, 3, 1, 2, 3]),
        Some((32, 16))
    );
    assert_eq!(sof_dimensions(&[8, 0x00, 0x10]), None);
}
