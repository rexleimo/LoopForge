use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn task_complete_def() -> ToolDefinition {
    function_def(
        "task_complete",
        "Mark a task as completed.",
        json!({
            "type": "object",
            "properties": {
                "task_id": { "type": "string", "description": "Task id." },
                "result": { "type": "string", "description": "Completion result summary." }
            },
            "required": ["task_id", "result"],
            "additionalProperties": false
        }),
    )
}
