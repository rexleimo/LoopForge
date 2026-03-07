use anyhow::bail;

use super::{tool_call_domain, ToolCallDomain};
use crate::Toolset;

impl Toolset {
    pub async fn call(&self, name: &str, arguments_json: &str) -> anyhow::Result<String> {
        self.ensure_tool_allowed(name)?;

        match tool_call_domain(name) {
            Some(ToolCallDomain::Fs) => self.call_fs_tool(name, arguments_json),
            Some(ToolCallDomain::Process) => self.call_process_tool(name, arguments_json).await,
            Some(ToolCallDomain::Web) => self.call_web_tool(name, arguments_json).await,
            Some(ToolCallDomain::Media) => self.call_media_tool(name, arguments_json),
            Some(ToolCallDomain::Browser) => self.call_browser_tool(name, arguments_json).await,
            Some(ToolCallDomain::RuntimeCompat) => Self::call_runtime_compat_tool(name),
            None => bail!("unknown tool: {name}"),
        }
    }

    fn ensure_tool_allowed(&self, name: &str) -> anyhow::Result<()> {
        if let Some(allowed) = self.allowed_tools.as_ref() {
            if !allowed.contains(name) {
                bail!("tool not allowed for this session: {name}");
            }
        }
        Ok(())
    }

    fn call_runtime_compat_tool(name: &str) -> anyhow::Result<String> {
        bail!("tool '{name}' is implemented in the runtime, not Toolset")
    }
}
