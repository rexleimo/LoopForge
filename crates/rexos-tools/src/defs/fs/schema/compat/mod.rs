mod list;
mod patch;
mod read;
mod write;

use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    vec![
        read::file_read_def(),
        write::file_write_def(),
        list::file_list_def(),
        patch::apply_patch_def(),
    ]
}
