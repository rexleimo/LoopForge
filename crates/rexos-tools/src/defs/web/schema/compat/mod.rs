mod a2a;
mod location;
mod search;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    vec![
        search::web_search_def(),
        location::location_get_def(),
        a2a::a2a_discover_def(),
        a2a::a2a_send_def(),
    ]
}
