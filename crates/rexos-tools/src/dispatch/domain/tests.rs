use super::browser::is_browser_tool;
use super::classify::tool_call_domain;
use super::runtime::is_runtime_compat_tool;
use super::ToolCallDomain;

#[test]
fn grouped_tool_classifiers_cover_browser_and_runtime_names() {
    assert!(is_browser_tool("browser_run_js"));
    assert!(is_runtime_compat_tool("cron_list"));
    assert!(is_runtime_compat_tool("workflow_run"));
    assert_eq!(tool_call_domain("pdf_extract"), Some(ToolCallDomain::Web));
    assert!(!is_browser_tool("cron_list"));
}
