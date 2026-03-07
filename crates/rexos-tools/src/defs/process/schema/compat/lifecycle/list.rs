use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn process_list_def() -> ToolDefinition {
    function_def(
        "process_list",
        "List running processes started via process_start.",
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        }),
    )
}
