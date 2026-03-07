pub(super) fn speech_to_text_payload(
    user_path: &str,
    text_value: &serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "path": user_path,
        "transcript": text_value.as_str().unwrap_or_default(),
        "text": text_value,
        "note": "MVP: speech_to_text currently supports transcript files (.txt/.md/.srt/.vtt).",
    })
}
