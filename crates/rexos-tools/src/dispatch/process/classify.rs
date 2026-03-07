#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ProcessDispatchKind {
    Exec,
    Managed,
    List,
}

pub(super) fn dispatch_kind(name: &str) -> Option<ProcessDispatchKind> {
    if is_exec_process_tool(name) {
        return Some(ProcessDispatchKind::Exec);
    }
    if is_managed_process_tool(name) {
        return Some(ProcessDispatchKind::Managed);
    }
    if is_process_list_tool(name) {
        return Some(ProcessDispatchKind::List);
    }
    None
}

pub(super) fn is_exec_process_tool(name: &str) -> bool {
    matches!(name, "shell" | "shell_exec" | "docker_exec")
}

pub(super) fn is_managed_process_tool(name: &str) -> bool {
    matches!(
        name,
        "process_start" | "process_poll" | "process_write" | "process_kill"
    )
}

pub(super) fn is_process_list_tool(name: &str) -> bool {
    matches!(name, "process_list")
}
