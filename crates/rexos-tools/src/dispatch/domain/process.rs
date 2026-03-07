pub(super) fn is_process_tool(name: &str) -> bool {
    matches!(
        name,
        "shell"
            | "shell_exec"
            | "docker_exec"
            | "process_start"
            | "process_poll"
            | "process_write"
            | "process_kill"
            | "process_list"
    )
}
