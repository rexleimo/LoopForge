use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::compat_function;

pub(super) fn image_generate_def() -> ToolDefinition {
    compat_function(
        "image_generate",
        "Generate an image asset from a prompt (currently outputs SVG).",
        json!({
            "type": "object",
            "properties": {
                "prompt": { "type": "string", "description": "Image generation prompt." },
                "path": { "type": "string", "description": "Workspace-relative output path (use .svg)." }
            },
            "required": ["prompt", "path"],
            "additionalProperties": false
        }),
    )
}
