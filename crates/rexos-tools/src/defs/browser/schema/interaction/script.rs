use rexos_llm::openai_compat::ToolDefinition;

use super::shared::function_def;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![browser_run_js_def()]
}

fn browser_run_js_def() -> ToolDefinition {
    function_def(
        "browser_run_js",
        "Run a JavaScript expression on the current page and return the result.",
        serde_json::json!({
            "type": "object",
            "properties": {
                "expression": { "type": "string", "description": "JavaScript expression to evaluate." }
            },
            "required": ["expression"],
            "additionalProperties": false
        }),
    )
}
