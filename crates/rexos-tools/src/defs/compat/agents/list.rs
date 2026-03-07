use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn agent_list_def() -> ToolDefinition {
    compat_function_def(
        "agent_list",
        "List known agent sessions.",
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        }),
    )
}
