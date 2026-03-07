pub(super) fn normalize_allowed_tools(tools: Vec<String>) -> std::collections::HashSet<String> {
    tools
        .into_iter()
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .collect()
}
