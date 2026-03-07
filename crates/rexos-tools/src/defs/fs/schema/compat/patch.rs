use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn apply_patch_def() -> ToolDefinition {
    function_def(
        "apply_patch",
        "Apply a multi-hunk diff patch to add, update, or delete files.",
        json!({
            "type": "object",
            "properties": {
                "patch": { "type": "string", "description": "Patch in *** Begin Patch / *** End Patch format." }
            },
            "required": ["patch"],
            "additionalProperties": false
        }),
    )
}
