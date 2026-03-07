use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn process_write_def() -> ToolDefinition {
    function_def(
        "process_write",
        "Write data to a running process's stdin (appends newline if missing).",
        json!({
            "type": "object",
            "properties": {
                "process_id": { "type": "string", "description": "Process id returned by process_start." },
                "data": { "type": "string", "description": "Data to write to stdin." }
            },
            "required": ["process_id", "data"],
            "additionalProperties": false
        }),
    )
}
