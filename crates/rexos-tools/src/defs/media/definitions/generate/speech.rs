use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::compat_function;

pub(super) fn text_to_speech_def() -> ToolDefinition {
    compat_function(
        "text_to_speech",
        "Convert text to speech audio (MVP: writes a short .wav).",
        json!({
            "type": "object",
            "properties": {
                "text": { "type": "string", "description": "Text to convert to speech." },
                "path": { "type": "string", "description": "Workspace-relative output path (use .wav). Optional." },
                "voice": { "type": "string", "description": "Optional voice name (ignored in MVP)." },
                "format": { "type": "string", "description": "Optional format (ignored in MVP; only .wav is supported)." }
            },
            "required": ["text"],
            "additionalProperties": false
        }),
    )
}
