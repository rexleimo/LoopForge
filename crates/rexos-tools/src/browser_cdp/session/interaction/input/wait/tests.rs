use super::polling::poll_count;
use super::validate::{clamped_timeout_ms, normalized_wait_args};

#[test]
fn normalized_wait_args_trim_blank_values() {
    let (selector, text) = normalized_wait_args(Some("  #id  "), Some("   "));
    assert_eq!(selector, Some("#id"));
    assert_eq!(text, None);
}

#[test]
fn clamped_timeout_ms_keeps_default_and_upper_bound() {
    assert_eq!(clamped_timeout_ms(None), 30_000);
    assert_eq!(clamped_timeout_ms(Some(99_999)), 30_000);
    assert_eq!(poll_count(30_000, 500), 60);
}
