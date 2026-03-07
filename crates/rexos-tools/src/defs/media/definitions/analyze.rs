use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![image_analyze_def(), media_describe_def()]
}

fn image_analyze_def() -> ToolDefinition {
    compat_function(
        "image_analyze",
        "Analyze an image file in the workspace (basic metadata).",
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Workspace-relative image path." }
            },
            "required": ["path"],
            "additionalProperties": false
        }),
    )
}

fn media_describe_def() -> ToolDefinition {
    compat_function(
        "media_describe",
        "Describe a media file in the workspace (best-effort metadata).",
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Workspace-relative media path." }
            },
            "required": ["path"],
            "additionalProperties": false
        }),
    )
}
