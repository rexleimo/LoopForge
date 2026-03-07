use super::boundary::{has_valid_prefix_boundary, skip_event_name_and_whitespace};

pub(super) fn contains_event_handler_attr(lower: &str) -> bool {
    let bytes = lower.as_bytes();
    for index in 0..bytes.len().saturating_sub(2) {
        if bytes[index] != b'o' || bytes[index + 1] != b'n' {
            continue;
        }

        if !has_valid_prefix_boundary(bytes, index) {
            continue;
        }

        let Some(cursor) = skip_event_name_and_whitespace(bytes, index + 2) else {
            continue;
        };

        if cursor < bytes.len() && bytes[cursor] == b'=' {
            return true;
        }
    }
    false
}
