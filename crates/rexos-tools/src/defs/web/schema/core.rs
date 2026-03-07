mod fetch;
mod pdf;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn core_tool_defs() -> Vec<ToolDefinition> {
    vec![
        fetch::web_fetch_def(),
        pdf::pdf_def(),
        pdf::pdf_extract_def(),
    ]
}
