use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn file_read_def() -> ToolDefinition {
    function_def(
        "file_read",
        "Read the contents of a file. Paths are relative to the agent workspace.",
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "The file path to read" }
            },
            "required": ["path"],
            "additionalProperties": false
        }),
    )
}
