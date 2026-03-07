pub(super) fn process_summary(
    process_id: &str,
    command: &str,
    args: &[String],
    exit_code: Option<i32>,
    uptime_secs: u64,
) -> serde_json::Value {
    serde_json::json!({
        "process_id": process_id,
        "command": command,
        "args": args,
        "alive": exit_code.is_none(),
        "exit_code": exit_code,
        "uptime_secs": uptime_secs,
    })
}
