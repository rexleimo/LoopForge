use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};

pub(super) fn browser_close_def() -> ToolDefinition {
    ToolDefinition {
        kind: "function".to_string(),
        function: ToolFunctionDefinition {
            name: "browser_close".to_string(),
            description: "Close the browser session (idempotent).".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false
            }),
        },
    }
}
