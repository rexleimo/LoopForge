use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::shared::compat_function_def;

pub(super) fn agent_spawn_def() -> ToolDefinition {
    compat_function_def(
        "agent_spawn",
        "Create an agent session record (persisted) and return its details.",
        json!({
            "type": "object",
            "properties": {
                "agent_id": { "type": "string", "description": "Optional stable agent id. If omitted, LoopForge generates one." },
                "name": { "type": "string", "description": "Optional human-friendly name." },
                "system_prompt": { "type": "string", "description": "Optional system prompt for the agent session." },
                "manifest_toml": { "type": "string", "description": "Optional agent manifest (TOML). LoopForge will best-effort extract name + system prompt." }
            },
            "additionalProperties": false
        }),
    )
}
