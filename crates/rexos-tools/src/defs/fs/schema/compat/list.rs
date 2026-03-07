use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn file_list_def() -> ToolDefinition {
    function_def(
        "file_list",
        "List files in a directory. Paths are relative to the agent workspace.",
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "The directory path to list" }
            },
            "required": ["path"],
            "additionalProperties": false
        }),
    )
}
