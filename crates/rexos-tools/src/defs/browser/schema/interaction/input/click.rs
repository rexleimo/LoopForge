use rexos_llm::openai_compat::ToolDefinition;

use super::super::shared::function_def;

pub(super) fn tool_def() -> ToolDefinition {
    function_def(
        "browser_click",
        "Click an element in the browser by CSS selector (or best-effort text fallback).",
        serde_json::json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "CSS selector (or text fallback) to click." }
            },
            "required": ["selector"],
            "additionalProperties": false
        }),
    )
}
