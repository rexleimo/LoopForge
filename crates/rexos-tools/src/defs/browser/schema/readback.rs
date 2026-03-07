use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![browser_read_page_def(), browser_screenshot_def()]
}

fn browser_read_page_def() -> ToolDefinition {
    ToolDefinition {
        kind: "function".to_string(),
        function: ToolFunctionDefinition {
            name: "browser_read_page".to_string(),
            description: "Read the current page content (title/url/text).".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false
            }),
        },
    }
}

fn browser_screenshot_def() -> ToolDefinition {
    ToolDefinition {
        kind: "function".to_string(),
        function: ToolFunctionDefinition {
            name: "browser_screenshot".to_string(),
            description: "Take a screenshot and write it to a workspace path.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string", "description": "Relative output path (default .loopforge/browser/screenshot.png)." }
                },
                "required": [],
                "additionalProperties": false
            }),
        },
    }
}
