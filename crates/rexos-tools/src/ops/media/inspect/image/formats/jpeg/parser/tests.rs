use super::cursor::next_marker;
use super::segment::segment_bounds;

#[test]
fn next_marker_skips_fill_bytes_and_returns_marker() {
    let bytes = [0x00, 0xFF, 0xFF, 0xC0, 0x00];
    let mut index = 0;

    assert_eq!(next_marker(&bytes, &mut index), Some(0xC0));
    assert_eq!(index, 4);
}

#[test]
fn segment_bounds_returns_data_range_and_advances_index() {
    let bytes = [0x00, 0x05, 1, 2, 3, 9];
    let mut index = 0;

    assert_eq!(segment_bounds(&bytes, &mut index), Some((2, 5)));
    assert_eq!(index, 5);
}
