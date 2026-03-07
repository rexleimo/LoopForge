use serde_json::json;

use super::payload::screenshot_result;
use super::read::parse_read_page_value;
use super::request::capture_screenshot_params;
use super::url::page_url_from_value;

#[test]
fn parse_read_page_value_decodes_json_strings() {
    assert_eq!(
        parse_read_page_value(json!("{\"title\":\"hello\"}")),
        json!({ "title": "hello" })
    );
}

#[test]
fn parse_read_page_value_preserves_non_json_text() {
    assert_eq!(
        parse_read_page_value(json!("plain text")),
        json!("plain text")
    );
}

#[test]
fn capture_screenshot_params_stays_png() {
    assert_eq!(capture_screenshot_params(), json!({ "format": "png" }));
}

#[test]
fn page_url_from_value_defaults_non_string_results() {
    assert_eq!(page_url_from_value(Some(json!(12))), String::new());
    assert_eq!(
        page_url_from_value(Some(json!("https://example.com"))),
        "https://example.com"
    );
}

#[test]
fn screenshot_result_keeps_expected_fields() {
    assert_eq!(
        screenshot_result("https://example.com".to_string(), "abc123"),
        json!({
            "format": "png",
            "url": "https://example.com",
            "image_base64": "abc123",
        })
    );
}
