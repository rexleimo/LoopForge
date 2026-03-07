use rexos_llm::openai_compat::ToolDefinition;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "browser_press_key",
        "Press a key in the browser (optionally on a target element).",
        serde_json::json!({
            "type": "object",
            "properties": {
                "key": { "type": "string", "description": "Key to press (example: Enter, Escape, ArrowDown, Control+A)." },
                "selector": { "type": "string", "description": "Optional CSS selector to target before pressing the key." }
            },
            "required": ["key"],
            "additionalProperties": false
        }),
    )
}
