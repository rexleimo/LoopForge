pub(super) fn is_supported_transcript_ext(ext: &str) -> bool {
    matches!(ext, "txt" | "md" | "srt" | "vtt")
}

pub(super) fn trimmed_transcript_text(raw: &str) -> String {
    raw.trim_end_matches(&['\r', '\n'][..]).to_string()
}
