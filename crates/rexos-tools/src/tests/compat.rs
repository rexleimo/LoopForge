use super::*;

#[test]
fn tool_definitions_include_compat_aliases_and_stubs() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let defs = tools
        .definitions()
        .into_iter()
        .map(|d| d.function.name)
        .collect::<std::collections::BTreeSet<_>>();

    for name in [
        "file_read",
        "file_write",
        "file_list",
        "apply_patch",
        "shell_exec",
        "web_search",
        "memory_store",
        "memory_recall",
        "agent_send",
        "task_post",
        "cron_create",
        "process_start",
        "canvas_present",
    ] {
        assert!(defs.contains(name), "missing tool definition: {name}");
    }
}

#[test]
fn tool_call_domain_classifies_core_and_compat_tools() {
    use super::dispatch::{tool_call_domain, ToolCallDomain};

    assert_eq!(tool_call_domain("fs_read"), Some(ToolCallDomain::Fs));
    assert_eq!(
        tool_call_domain("shell_exec"),
        Some(ToolCallDomain::Process)
    );
    assert_eq!(tool_call_domain("pdf_extract"), Some(ToolCallDomain::Web));
    assert_eq!(tool_call_domain("location_get"), Some(ToolCallDomain::Web));
    assert_eq!(
        tool_call_domain("image_generate"),
        Some(ToolCallDomain::Media)
    );
    assert_eq!(
        tool_call_domain("browser_run_js"),
        Some(ToolCallDomain::Browser)
    );
    assert_eq!(
        tool_call_domain("workflow_run"),
        Some(ToolCallDomain::RuntimeCompat)
    );
    assert_eq!(tool_call_domain("unknown_tool"), None);
}

#[tokio::test]
async fn runtime_tools_are_reported_as_runtime_implemented() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let err = tools.call("agent_send", r#"{}"#).await.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("runtime"), "{msg}");
}

#[tokio::test]
async fn hand_tools_are_reported_as_runtime_implemented() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let err = tools.call("hand_list", r#"{}"#).await.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("runtime"), "{msg}");
}

#[tokio::test]
async fn workflow_run_is_reported_as_runtime_implemented() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();
    let err = tools
        .call("workflow_run", r#"{ "steps": [] }"#)
        .await
        .unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("runtime"), "{msg}");
}
