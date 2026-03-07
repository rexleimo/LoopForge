use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn process_kill_def() -> ToolDefinition {
    function_def(
        "process_kill",
        "Terminate a running process and clean up resources.",
        json!({
            "type": "object",
            "properties": {
                "process_id": { "type": "string", "description": "Process id returned by process_start." }
            },
            "required": ["process_id"],
            "additionalProperties": false
        }),
    )
}
