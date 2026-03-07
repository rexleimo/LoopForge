pub(super) fn process_poll_payload(
    stdout: &str,
    stderr: &str,
    stdout_truncated: bool,
    stderr_truncated: bool,
    exit_code: Option<i32>,
    alive: bool,
) -> serde_json::Value {
    serde_json::json!({
        "stdout": stdout,
        "stderr": stderr,
        "stdout_truncated": stdout_truncated,
        "stderr_truncated": stderr_truncated,
        "exit_code": exit_code,
        "alive": alive,
    })
}
