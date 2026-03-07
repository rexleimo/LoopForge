use super::classify::{
    dispatch_kind, is_exec_process_tool, is_managed_process_tool, is_process_list_tool,
    ProcessDispatchKind,
};

#[test]
fn grouped_process_tool_classifiers_cover_exec_and_managed_names() {
    assert!(is_exec_process_tool("shell_exec"));
    assert!(is_managed_process_tool("process_poll"));
    assert!(is_process_list_tool("process_list"));
    assert_eq!(
        dispatch_kind("process_list"),
        Some(ProcessDispatchKind::List)
    );
    assert!(!is_exec_process_tool("process_poll"));
}
