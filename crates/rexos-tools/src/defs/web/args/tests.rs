use super::{A2aSendArgs, PdfArgs, WebFetchArgs};

#[test]
fn web_fetch_args_deserialize_optional_fields_with_defaults() {
    let args: WebFetchArgs = serde_json::from_value(serde_json::json!({
        "url": "https://example.com"
    }))
    .unwrap();

    assert_eq!(args.url, "https://example.com");
    assert_eq!(args.timeout_ms, None);
    assert_eq!(args.max_bytes, None);
    assert!(!args.allow_private);
}

#[test]
fn pdf_and_a2a_send_args_keep_optional_fields_nullable() {
    let pdf: PdfArgs = serde_json::from_value(serde_json::json!({
        "path": "doc.pdf"
    }))
    .unwrap();
    assert_eq!(pdf.path, "doc.pdf");
    assert_eq!(pdf.pages, None);
    assert_eq!(pdf.max_pages, None);
    assert_eq!(pdf.max_chars, None);

    let a2a: A2aSendArgs = serde_json::from_value(serde_json::json!({
        "message": "hello"
    }))
    .unwrap();
    assert_eq!(a2a.agent_url, None);
    assert_eq!(a2a.url, None);
    assert_eq!(a2a.message, "hello");
    assert_eq!(a2a.session_id, None);
    assert!(!a2a.allow_private);
}
