use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};

pub(super) fn browser_navigate_def() -> ToolDefinition {
    ToolDefinition {
        kind: "function".to_string(),
        function: ToolFunctionDefinition {
            name: "browser_navigate".to_string(),
            description: "Navigate the browser to a URL (SSRF-protected by default).".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": { "type": "string", "description": "HTTP(S) URL to open." },
                    "timeout_ms": { "type": "integer", "description": "Timeout in milliseconds (default 30000).", "minimum": 1 },
                    "allow_private": { "type": "boolean", "description": "Allow loopback/private IPs (default false)." },
                    "headless": { "type": "boolean", "description": "Run the browser in headless mode (default true). Set false to show a GUI window." }
                },
                "required": ["url"],
                "additionalProperties": false
            }),
        },
    }
}
