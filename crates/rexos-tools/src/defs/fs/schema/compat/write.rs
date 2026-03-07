use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn file_write_def() -> ToolDefinition {
    function_def(
        "file_write",
        "Write content to a file. Paths are relative to the agent workspace.",
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "The file path to write to" },
                "content": { "type": "string", "description": "The content to write" }
            },
            "required": ["path", "content"],
            "additionalProperties": false
        }),
    )
}
