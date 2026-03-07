use std::sync::Arc;

#[test]
fn unknown_process_id_error_mentions_requested_id() {
    let err = super::entry::unknown_process_id_error("proc-404");
    assert!(err.to_string().contains("proc-404"), "{err}");
}

#[tokio::test]
async fn take_buffer_text_drains_contents_and_reports_not_truncated() {
    let buffer = Arc::new(tokio::sync::Mutex::new(
        crate::process_runtime::ProcessOutputBuffer::default(),
    ));
    {
        let mut guard = buffer.lock().await;
        guard.push(b"hello world");
    }

    let (text, truncated) = super::output::take_buffer_text(&buffer).await;
    assert_eq!(text, "hello world");
    assert!(!truncated);
}
