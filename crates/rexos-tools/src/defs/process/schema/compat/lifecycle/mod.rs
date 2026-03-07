mod kill;
mod list;
mod poll;
mod start;
mod write;

use rexos_llm::openai_compat::ToolDefinition;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![
        start::process_start_def(),
        poll::process_poll_def(),
        write::process_write_def(),
        kill::process_kill_def(),
        list::process_list_def(),
    ]
}
