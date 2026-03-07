use super::take_text;
use crate::process_runtime::ProcessOutputBuffer;

#[test]
fn reconstruct_all_bytes_keeps_non_overlapping_suffix_once() {
    let buffer = ProcessOutputBuffer {
        head: b"abcdef".to_vec(),
        tail: b"efXYZ".to_vec(),
        total_bytes: 9,
    };
    assert_eq!(buffer.reconstruct_all_bytes(), b"abcdefXYZ".to_vec());
}

#[test]
fn take_text_clears_buffer_state_after_read() {
    let mut buffer = ProcessOutputBuffer {
        head: b"hello".to_vec(),
        tail: b"hello".to_vec(),
        total_bytes: 5,
    };
    let (text, truncated) = take_text(&mut buffer);
    assert_eq!(text, "hello");
    assert!(!truncated);
    assert!(buffer.head.is_empty());
    assert!(buffer.tail.is_empty());
    assert_eq!(buffer.total_bytes, 0);
}
