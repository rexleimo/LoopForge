use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn location_get_def() -> ToolDefinition {
    function_def(
        "location_get",
        "Get environment location metadata (os/arch/tz).",
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        }),
    )
}
