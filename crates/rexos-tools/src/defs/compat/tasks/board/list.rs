use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn task_list_def() -> ToolDefinition {
    function_def(
        "task_list",
        "List tasks (optionally filtered by status).",
        json!({
            "type": "object",
            "properties": {
                "status": { "type": "string", "description": "Optional filter: pending | claimed | completed." }
            },
            "additionalProperties": false
        }),
    )
}
