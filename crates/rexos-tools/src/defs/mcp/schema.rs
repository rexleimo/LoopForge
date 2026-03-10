use rexos_llm::openai_compat::{ToolDefinition, ToolFunctionDefinition};
use serde_json::{json, Value};

pub(crate) fn wrapper_tool_defs() -> Vec<ToolDefinition> {
    vec![
        mcp_servers_list_def(),
        mcp_resources_list_def(),
        mcp_resources_read_def(),
        mcp_prompts_list_def(),
        mcp_prompts_get_def(),
    ]
}

fn function_def(name: &str, description: &str, parameters: Value) -> ToolDefinition {
    ToolDefinition {
        kind: "function".to_string(),
        function: ToolFunctionDefinition {
            name: name.to_string(),
            description: description.to_string(),
            parameters,
        },
    }
}

fn mcp_servers_list_def() -> ToolDefinition {
    function_def(
        "mcp_servers_list",
        "List MCP servers configured for this session.",
        json!({
            "type": "object",
            "properties": {},
            "additionalProperties": false
        }),
    )
}

fn mcp_resources_list_def() -> ToolDefinition {
    function_def(
        "mcp_resources_list",
        "List MCP resources (optionally for one server).",
        json!({
            "type": "object",
            "properties": {
                "server": { "type": "string", "description": "Optional server name. When omitted, lists resources for all servers." },
                "cursor": { "type": "string", "description": "Optional pagination cursor." }
            },
            "additionalProperties": false
        }),
    )
}

fn mcp_resources_read_def() -> ToolDefinition {
    function_def(
        "mcp_resources_read",
        "Read an MCP resource by URI (optionally from one server).",
        json!({
            "type": "object",
            "properties": {
                "server": { "type": "string", "description": "Optional server name. When omitted, tries all servers." },
                "uri": { "type": "string", "description": "Resource URI." }
            },
            "required": ["uri"],
            "additionalProperties": false
        }),
    )
}

fn mcp_prompts_list_def() -> ToolDefinition {
    function_def(
        "mcp_prompts_list",
        "List MCP prompts (optionally for one server).",
        json!({
            "type": "object",
            "properties": {
                "server": { "type": "string", "description": "Optional server name. When omitted, lists prompts for all servers." },
                "cursor": { "type": "string", "description": "Optional pagination cursor." }
            },
            "additionalProperties": false
        }),
    )
}

fn mcp_prompts_get_def() -> ToolDefinition {
    function_def(
        "mcp_prompts_get",
        "Fetch an MCP prompt by name (optionally from one server).",
        json!({
            "type": "object",
            "properties": {
                "server": { "type": "string", "description": "Optional server name. When omitted, tries all servers." },
                "name": { "type": "string", "description": "Prompt name." },
                "arguments": { "type": "object", "description": "Optional prompt arguments." }
            },
            "required": ["name"],
            "additionalProperties": false
        }),
    )
}
