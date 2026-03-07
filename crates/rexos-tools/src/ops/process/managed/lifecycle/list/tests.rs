#[test]
fn process_summary_reports_alive_and_exit_code() {
    let value = super::summary::process_summary(
        "proc-1",
        "bash",
        &["-lc".to_string(), "echo hi".to_string()],
        None,
        12,
    );

    assert_eq!(
        value.get("process_id").and_then(|v| v.as_str()),
        Some("proc-1")
    );
    assert_eq!(value.get("command").and_then(|v| v.as_str()), Some("bash"));
    assert_eq!(value.get("alive").and_then(|v| v.as_bool()), Some(true));
    assert_eq!(value.get("exit_code"), Some(&serde_json::Value::Null));
    assert_eq!(value.get("uptime_secs").and_then(|v| v.as_u64()), Some(12));
}

#[test]
fn process_summary_marks_exited_processes_not_alive() {
    let value = super::summary::process_summary("proc-2", "bash", &[], Some(9), 3);

    assert_eq!(value.get("alive").and_then(|v| v.as_bool()), Some(false));
    assert_eq!(value.get("exit_code").and_then(|v| v.as_i64()), Some(9));
}
