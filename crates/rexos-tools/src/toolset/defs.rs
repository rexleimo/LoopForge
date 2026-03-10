use rexos_llm::openai_compat::ToolDefinition;

use super::Toolset;
use crate::defs::{compat_tool_defs, core_tool_defs, mcp_wrapper_tool_defs};

impl Toolset {
    pub fn definitions(&self) -> Vec<ToolDefinition> {
        let mut defs = core_tool_defs();
        defs.extend(compat_tool_defs());
        if let Some(mcp) = self.mcp.as_ref() {
            defs.extend(mcp_wrapper_tool_defs());
            defs.extend(mcp.tool_definitions().iter().cloned());
        }
        if let Some(allowed) = self.allowed_tools.as_ref() {
            defs.retain(|def| allowed.contains(def.function.name.as_str()));
        }
        defs
    }
}
