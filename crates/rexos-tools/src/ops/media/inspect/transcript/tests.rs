use serde_json::json;

use super::shared::trimmed_transcript_text;
use super::speech::speech_to_text_payload;

#[test]
fn trimmed_transcript_text_strips_trailing_newlines_only() {
    assert_eq!(trimmed_transcript_text("hello\n\n"), "hello");
    assert_eq!(trimmed_transcript_text("hello  "), "hello  ");
}

#[test]
fn speech_to_text_payload_keeps_text_and_transcript_fields_aligned() {
    assert_eq!(
        speech_to_text_payload("notes.txt", &json!("hello")),
        json!({
            "path": "notes.txt",
            "transcript": "hello",
            "text": "hello",
            "note": "MVP: speech_to_text currently supports transcript files (.txt/.md/.srt/.vtt).",
        })
    );
}
