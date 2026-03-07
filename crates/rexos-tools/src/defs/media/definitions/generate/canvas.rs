use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::compat_function;

pub(super) fn canvas_present_def() -> ToolDefinition {
    compat_function(
        "canvas_present",
        "Present sanitized HTML as a canvas artifact (saved to workspace output/).",
        json!({
            "type": "object",
            "properties": {
                "html": { "type": "string", "description": "HTML content to present (scripts/event handlers are forbidden)." },
                "title": { "type": "string", "description": "Optional canvas title." }
            },
            "required": ["html"],
            "additionalProperties": false
        }),
    )
}
