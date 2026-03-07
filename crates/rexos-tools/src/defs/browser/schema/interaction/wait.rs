use rexos_llm::openai_compat::ToolDefinition;

use super::shared::function_def;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![browser_wait_def(), browser_wait_for_def()]
}

fn browser_wait_def() -> ToolDefinition {
    function_def(
        "browser_wait",
        "Wait for a CSS selector to appear on the page.",
        serde_json::json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "CSS selector to wait for." },
                "timeout_ms": { "type": "integer", "description": "Optional timeout in milliseconds.", "minimum": 1 }
            },
            "required": ["selector"],
            "additionalProperties": false
        }),
    )
}

fn browser_wait_for_def() -> ToolDefinition {
    function_def(
        "browser_wait_for",
        "Wait for a selector or text to appear on the page.",
        serde_json::json!({
            "type": "object",
            "properties": {
                "selector": { "type": "string", "description": "Optional CSS selector to wait for." },
                "text": { "type": "string", "description": "Optional visible text to wait for." },
                "timeout_ms": { "type": "integer", "description": "Optional timeout in milliseconds.", "minimum": 1 }
            },
            "additionalProperties": false
        }),
    )
}
