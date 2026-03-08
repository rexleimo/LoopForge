pub(crate) fn permission_tools(permissions: &[String]) -> Vec<String> {
    let mut tools = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for raw in permissions {
        let p = raw.trim().to_ascii_lowercase();
        if p.is_empty() {
            continue;
        }
        if p == "readonly" {
            for tool in ["fs_read", "fs_list", "web_search", "web_fetch"] {
                if seen.insert(tool.to_string()) {
                    tools.push(tool.to_string());
                }
            }
            continue;
        }

        if let Some(tool) = p.strip_prefix("tool:") {
            let tool = tool.trim();
            if !tool.is_empty() && seen.insert(tool.to_string()) {
                tools.push(tool.to_string());
            }
        }
    }

    tools
}
