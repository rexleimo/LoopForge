use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn hand_list_def() -> ToolDefinition {
    compat_function_def(
        "hand_list",
        "List available Hands (curated autonomous packages) and their activation status.",
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        }),
    )
}
