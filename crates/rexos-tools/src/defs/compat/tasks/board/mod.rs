mod claim;
mod complete;
mod list;
mod post;

use rexos_llm::openai_compat::ToolDefinition;

pub(super) fn task_tool_defs() -> Vec<ToolDefinition> {
    vec![
        post::task_post_def(),
        list::task_list_def(),
        claim::task_claim_def(),
        complete::task_complete_def(),
    ]
}
