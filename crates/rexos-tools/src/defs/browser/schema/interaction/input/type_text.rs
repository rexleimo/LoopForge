use rexos_llm::openai_compat::ToolDefinition;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "browser_type",
        "Type into an input element in the browser (fills the field).",
        serde_json::json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "CSS selector for the input element." },
                "text": { "type": "string", "description": "Text to input." }
            },
            "required": ["selector", "text"],
            "additionalProperties": false
        }),
    )
}
