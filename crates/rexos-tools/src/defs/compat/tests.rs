use std::collections::BTreeSet;

use super::compat_tool_defs;

#[test]
fn compat_tool_defs_have_unique_names() {
    let defs = compat_tool_defs();
    let names: Vec<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();
    let unique: BTreeSet<&str> = names.iter().copied().collect();
    assert_eq!(
        names.len(),
        unique.len(),
        "duplicate compat tool names: {names:?}"
    );
}

#[test]
fn compat_tool_defs_cover_runtime_domains() {
    let defs = compat_tool_defs();
    let names: BTreeSet<&str> = defs.iter().map(|def| def.function.name.as_str()).collect();
    for name in [
        "memory_store",
        "agent_spawn",
        "task_post",
        "schedule_create",
        "knowledge_query",
        "workflow_run",
        "hand_activate",
    ] {
        assert!(
            names.contains(name),
            "missing compat tool definition: {name}"
        );
    }
}
