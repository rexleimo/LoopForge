use super::{browser, fs, mcp, media, process, runtime, web, ToolCallDomain};

pub(super) fn tool_call_domain(name: &str) -> Option<ToolCallDomain> {
    if mcp::is_mcp_tool(name) {
        return Some(ToolCallDomain::Mcp);
    }
    if fs::is_fs_tool(name) {
        return Some(ToolCallDomain::Fs);
    }
    if process::is_process_tool(name) {
        return Some(ToolCallDomain::Process);
    }
    if web::is_web_tool(name) {
        return Some(ToolCallDomain::Web);
    }
    if media::is_media_tool(name) {
        return Some(ToolCallDomain::Media);
    }
    if browser::is_browser_tool(name) {
        return Some(ToolCallDomain::Browser);
    }
    if runtime::is_runtime_compat_tool(name) {
        return Some(ToolCallDomain::RuntimeCompat);
    }
    None
}
