use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![media_transcribe_def(), speech_to_text_def()]
}

fn media_transcribe_def() -> ToolDefinition {
    compat_function(
        "media_transcribe",
        "Transcribe media into text (currently supports text transcript files).",
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Workspace-relative transcript path (.txt/.md/.srt/.vtt)." }
            },
            "required": ["path"],
            "additionalProperties": false
        }),
    )
}

fn speech_to_text_def() -> ToolDefinition {
    compat_function(
        "speech_to_text",
        "Transcribe speech/audio into text (MVP: supports transcript files).",
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Workspace-relative transcript path (.txt/.md/.srt/.vtt)." }
            },
            "required": ["path"],
            "additionalProperties": false
        }),
    )
}
