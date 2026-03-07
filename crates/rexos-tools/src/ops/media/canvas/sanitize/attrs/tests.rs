use super::boundary::{has_valid_prefix_boundary, skip_event_name_and_whitespace};
use super::scan::contains_event_handler_attr;

#[test]
fn contains_event_handler_attr_detects_assignment_after_event_name() {
    assert!(contains_event_handler_attr(
        "<img src=x onerror = alert(1)>"
    ));
}

#[test]
fn contains_event_handler_attr_ignores_embedded_on_words_without_boundary() {
    assert!(!contains_event_handler_attr(
        "<div data-onclick=\"x\">ok</div>"
    ));
    assert!(has_valid_prefix_boundary(b" onclick=", 1));
    assert_eq!(skip_event_name_and_whitespace(b"click =x", 0), Some(6));
}
