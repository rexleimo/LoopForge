#[path = "builder/allowed.rs"]
mod allowed;
#[path = "builder/client.rs"]
mod client;
#[cfg(test)]
#[path = "builder/tests.rs"]
mod tests;

use anyhow::Context;
use rexos_kernel::security::SecurityConfig;

use super::Toolset;
use crate::process_runtime::ProcessManager;

impl Toolset {
    pub fn new(workspace_root: std::path::PathBuf) -> anyhow::Result<Self> {
        Self::new_with_allowed_tools_and_security(workspace_root, None, SecurityConfig::default())
    }

    pub fn new_with_security_config(
        workspace_root: std::path::PathBuf,
        security: SecurityConfig,
    ) -> anyhow::Result<Self> {
        Self::new_with_allowed_tools_and_security(workspace_root, None, security)
    }

    pub fn new_with_allowed_tools(
        workspace_root: std::path::PathBuf,
        allowed_tools: Option<Vec<String>>,
    ) -> anyhow::Result<Self> {
        Self::new_with_allowed_tools_and_security(
            workspace_root,
            allowed_tools,
            SecurityConfig::default(),
        )
    }

    pub fn new_with_allowed_tools_and_security(
        workspace_root: std::path::PathBuf,
        allowed_tools: Option<Vec<String>>,
        security: SecurityConfig,
    ) -> anyhow::Result<Self> {
        let workspace_root = workspace_root.canonicalize().with_context(|| {
            format!("canonicalize workspace root: {}", workspace_root.display())
        })?;
        let http = client::build_http_client()?;

        Ok(Self {
            workspace_root,
            http,
            browser: std::sync::Arc::new(tokio::sync::Mutex::new(None)),
            processes: std::sync::Arc::new(tokio::sync::Mutex::new(ProcessManager::new())),
            allowed_tools: allowed_tools.map(allowed::normalize_allowed_tools),
            security,
        })
    }
}
