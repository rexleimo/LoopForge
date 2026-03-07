use reqwest::StatusCode;

use super::status::ensure_success_status;
use super::validate::{ensure_non_empty_message, parse_agent_url};

#[test]
fn ensure_non_empty_message_rejects_blank_input() {
    let err = ensure_non_empty_message("   ").unwrap_err();
    assert!(err.to_string().contains("message is empty"), "{err}");
}

#[test]
fn parse_agent_url_rejects_invalid_urls() {
    let err = parse_agent_url("not a url").unwrap_err();
    assert!(err.to_string().contains("parse agent_url"), "{err}");
}

#[test]
fn ensure_success_status_rejects_http_failures() {
    let err = ensure_success_status(StatusCode::BAD_GATEWAY).unwrap_err();
    assert!(
        err.to_string().contains("a2a_send http 502 Bad Gateway"),
        "{err}"
    );
}
