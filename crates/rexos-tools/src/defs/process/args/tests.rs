use super::{ProcessStartArgs, ShellArgs, ShellExecArgs};

#[test]
fn shell_args_deserialize_optional_timeout_ms() {
    let args: ShellArgs = serde_json::from_str(r#"{ "command": "echo hi" }"#).unwrap();
    assert_eq!(args.command, "echo hi");
    assert_eq!(args.timeout_ms, None);
}

#[test]
fn shell_exec_args_deserialize_optional_timeout_seconds() {
    let args: ShellExecArgs =
        serde_json::from_str(r#"{ "command": "bash", "timeout_seconds": 5 }"#).unwrap();
    assert_eq!(args.command, "bash");
    assert_eq!(args.timeout_seconds, Some(5));
}

#[test]
fn process_start_args_default_args_to_empty_list() {
    let args: ProcessStartArgs = serde_json::from_str(r#"{ "command": "python" }"#).unwrap();
    assert_eq!(args.command, "python");
    assert!(args.args.is_empty());
}
