use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "cron_list",
        "List cron/scheduled job records.",
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        }),
    )
}
