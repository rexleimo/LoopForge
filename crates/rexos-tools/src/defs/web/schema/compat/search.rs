use rexos_llm::openai_compat::ToolDefinition;
use serde_json::json;

use super::super::shared::function_def;

pub(super) fn web_search_def() -> ToolDefinition {
    function_def(
        "web_search",
        "Search the web and return a short list of results.",
        json!({
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "The search query" },
                "max_results": { "type": "integer", "description": "Maximum number of results to return (default: 5, max: 20)" }
            },
            "required": ["query"],
            "additionalProperties": false
        }),
    )
}
