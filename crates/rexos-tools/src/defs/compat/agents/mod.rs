mod find;
mod kill;
mod list;
mod send;
mod shared;
mod spawn;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    vec![
        spawn::agent_spawn_def(),
        list::agent_list_def(),
        find::agent_find_def(),
        kill::agent_kill_def(),
        send::agent_send_def(),
    ]
}
