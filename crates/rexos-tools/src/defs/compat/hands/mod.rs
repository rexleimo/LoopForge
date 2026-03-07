mod activate;
mod deactivate;
mod list;
mod shared;
mod status;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    vec![
        list::hand_list_def(),
        activate::hand_activate_def(),
        status::hand_status_def(),
        deactivate::hand_deactivate_def(),
    ]
}
