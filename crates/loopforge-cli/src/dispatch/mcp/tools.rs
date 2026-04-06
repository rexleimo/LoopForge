pub(super) fn list_remote_tool_names(tools: &rexos::tools::Toolset) -> Vec<String> {
    let mut names: Vec<String> = tools
        .definitions()
        .into_iter()
        .filter_map(|def| {
            let name = def.function.name;
            if name.starts_with("mcp_") && name.contains("__") {
                Some(name)
            } else {
                None
            }
        })
        .collect();
    names.sort();
    names
}
