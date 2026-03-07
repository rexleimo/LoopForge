use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "schedule_list",
        "List schedule entries.",
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        }),
    )
}
