use serde_json::json;

use super::limits::{clamped_max_chars, clamped_max_pages};
use super::response::pdf_extract_payload;

#[test]
fn pdf_extract_limit_helpers_apply_defaults_and_bounds() {
    assert_eq!(clamped_max_pages(None), 10);
    assert_eq!(clamped_max_pages(Some(0)), 1);
    assert_eq!(clamped_max_pages(Some(99)), 50);
    assert_eq!(clamped_max_chars(None), 12_000);
    assert_eq!(clamped_max_chars(Some(0)), 1);
    assert_eq!(clamped_max_chars(Some(90_000)), 50_000);
}

#[test]
fn pdf_extract_payload_reports_selected_page_metadata() {
    assert_eq!(
        pdf_extract_payload("doc.pdf", "hello", true, 123, 8, Some("1-2"), 2),
        json!({
            "path": "doc.pdf",
            "text": "hello",
            "truncated": true,
            "bytes": 123,
            "pages_total": 8,
            "pages": "1-2",
            "pages_extracted": 2,
        })
    );
}
