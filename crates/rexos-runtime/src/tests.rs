use std::path::Path;

use crate::leak_guard::LeakGuardAudit;
use crate::runtime_utils::{is_runtime_managed_tool, tool_event_payload, workflow_state_path};

#[test]
fn tool_event_payload_keeps_expected_fields() {
    let payload = tool_event_payload(
        "shell",
        Some(true),
        Some("boom"),
        Some("policy"),
        Some(&LeakGuardAudit {
            mode: "warn".to_string(),
            detectors: vec!["token:sk".to_string()],
            redacted: false,
            blocked: false,
        }),
    );

    assert_eq!(payload["tool"], "shell");
    assert_eq!(payload["truncated"], true);
    assert_eq!(payload["error"], "boom");
    assert_eq!(payload["reason"], "policy");
    assert_eq!(payload["leak_guard"]["mode"], "warn");
    assert_eq!(payload["leak_guard"]["detectors"][0], "token:sk");
}

#[test]
fn runtime_utils_report_expected_workflow_paths_and_tools() {
    let path = workflow_state_path(Path::new("/tmp/workspace"), "wf-123");
    assert!(
        path.ends_with(".loopforge/workflows/wf-123.json"),
        "{}",
        path.display()
    );

    assert!(is_runtime_managed_tool("workflow_run"));
    assert!(is_runtime_managed_tool("channel_send"));
    assert!(!is_runtime_managed_tool("fs_read"));
}
