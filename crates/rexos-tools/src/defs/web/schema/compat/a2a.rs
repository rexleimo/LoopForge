use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn a2a_discover_def() -> ToolDefinition {
    function_def(
        "a2a_discover",
        "Discover an external A2A agent by fetching its agent card at `/.well-known/agent.json`.",
        json!({
            "type": "object",
            "properties": {
                "url": { "type": "string", "description": "Base URL of the remote agent (http/https)." },
                "allow_private": { "type": "boolean", "description": "Allow loopback/private IPs (default false)." }
            },
            "required": ["url"],
            "additionalProperties": false
        }),
    )
}

pub(super) fn a2a_send_def() -> ToolDefinition {
    function_def(
        "a2a_send",
        "Send a JSON-RPC `tasks/send` request to an external A2A agent endpoint.",
        json!({
            "type": "object",
            "properties": {
                "agent_url": { "type": "string", "description": "Full JSON-RPC endpoint URL (http/https)." },
                "url": { "type": "string", "description": "Alias for agent_url." },
                "message": { "type": "string", "description": "Message to send to the remote agent." },
                "session_id": { "type": "string", "description": "Optional session id for continuity." },
                "allow_private": { "type": "boolean", "description": "Allow loopback/private IPs (default false)." }
            },
            "required": ["message"],
            "additionalProperties": false
        }),
    )
}
