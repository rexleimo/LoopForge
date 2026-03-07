pub(super) fn pdf_extract_payload(
    user_path: &str,
    text: &str,
    truncated: bool,
    bytes: u64,
    total_pages: usize,
    pages_spec: Option<&str>,
    pages_extracted: usize,
) -> serde_json::Value {
    serde_json::json!({
        "path": user_path,
        "text": text,
        "truncated": truncated,
        "bytes": bytes,
        "pages_total": total_pages,
        "pages": pages_spec,
        "pages_extracted": pages_extracted,
    })
}
