use std::collections::HashSet;

pub(crate) fn tool_approval_is_granted(tool_name: &str) -> bool {
    let raw = std::env::var("LOOPFORGE_APPROVAL_ALLOW").unwrap_or_default();
    let items: HashSet<String> = raw
        .split(',')
        .map(|entry| entry.trim().to_lowercase())
        .filter(|entry| !entry.is_empty())
        .collect();
    if items.contains("all") {
        return true;
    }
    items.contains(&tool_name.to_lowercase())
}

pub(crate) fn skill_approval_is_granted(skill_name: &str) -> bool {
    let raw = std::env::var("LOOPFORGE_SKILL_APPROVAL_ALLOW").unwrap_or_default();
    let items: HashSet<String> = raw
        .split(',')
        .map(|entry| entry.trim().to_lowercase())
        .filter(|entry| !entry.is_empty())
        .collect();
    if items.contains("all") {
        return true;
    }
    items.contains(&skill_name.to_lowercase())
}

pub(crate) fn skill_permissions_are_readonly(permissions: &[String]) -> bool {
    if permissions.is_empty() {
        return true;
    }

    for raw in permissions {
        let permission = raw.trim().to_ascii_lowercase();
        if permission.is_empty() {
            continue;
        }
        if permission == "readonly" {
            continue;
        }
        if permission.starts_with("tool:") {
            let tool = permission.trim_start_matches("tool:");
            let dangerous = [
                "shell",
                "docker_exec",
                "fs_write",
                "apply_patch",
                "process_start",
                "browser_navigate",
                "web_fetch",
            ];
            if dangerous.contains(&tool) {
                return false;
            }
            continue;
        }
        if permission.contains("write")
            || permission.contains("patch")
            || permission.contains("delete")
            || permission.contains("shell")
            || permission.contains("docker")
            || permission.contains("network")
            || permission.contains("process")
        {
            return false;
        }
    }

    true
}
