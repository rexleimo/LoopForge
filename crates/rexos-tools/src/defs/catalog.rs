use rexos_llm::openai_compat::ToolDefinition;

pub(crate) fn core_tool_defs() -> Vec<ToolDefinition> {
    let mut defs = Vec::new();
    defs.extend(super::fs::core_tool_defs());
    defs.extend(super::process::core_tool_defs());
    defs.extend(super::web::core_tool_defs());
    defs.extend(super::media::core_tool_defs());
    defs.extend(super::browser::core_tool_defs());
    defs
}

pub(crate) fn compat_tool_defs() -> Vec<ToolDefinition> {
    let mut defs = Vec::new();
    defs.extend(super::fs::compat_tool_defs());
    defs.extend(super::process::compat_tool_defs());
    defs.extend(super::web::compat_tool_defs());
    defs.extend(super::media::compat_tool_defs());
    defs.extend(super::browser::compat_tool_defs());
    defs.extend(super::compat::compat_tool_defs());
    defs
}
