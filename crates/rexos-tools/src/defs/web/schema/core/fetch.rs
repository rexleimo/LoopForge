use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn web_fetch_def() -> ToolDefinition {
    function_def(
        "web_fetch",
        "Fetch a URL via HTTP(S) and return a small response body (SSRF-protected).",
        json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "HTTP(S) URL to fetch." },
                "timeout_ms": { "type": "integer", "description": "Timeout in milliseconds (default 20000).", "minimum": 1 },
                "max_bytes": { "type": "integer", "description": "Maximum bytes to return (default 200000).", "minimum": 1 },
                "allow_private": { "type": "boolean", "description": "Allow fetching loopback/private IPs (default false)." }
            },
            "required": ["url"],
            "additionalProperties": false
        }),
    )
}
