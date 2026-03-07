pub(super) fn is_media_tool(name: &str) -> bool {
    matches!(
        name,
        "image_analyze"
            | "media_describe"
            | "media_transcribe"
            | "speech_to_text"
            | "text_to_speech"
            | "image_generate"
            | "canvas_present"
    )
}
