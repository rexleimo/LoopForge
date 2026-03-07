mod back;
mod close;
mod navigate;
mod scroll;

use rexos_llm::openai_compat::ToolDefinition;

pub(super) fn tool_defs() -> Vec<ToolDefinition> {
    vec![
        navigate::browser_navigate_def(),
        back::browser_back_def(),
        scroll::browser_scroll_def(),
        close::browser_close_def(),
    ]
}
