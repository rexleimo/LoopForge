mod builder;
mod defs;

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use crate::browser_runtime::BrowserSession;
use crate::mcp::McpHub;
use crate::process_runtime::ProcessManager;
use rexos_kernel::security::SecurityConfig;

#[derive(Debug, Clone)]
pub struct Toolset {
    pub(crate) workspace_root: PathBuf,
    pub(crate) http: reqwest::Client,
    pub(crate) browser: Arc<tokio::sync::Mutex<Option<BrowserSession>>>,
    pub(crate) processes: Arc<tokio::sync::Mutex<ProcessManager>>,
    pub(crate) allowed_tools: Option<HashSet<String>>,
    pub(crate) security: SecurityConfig,
    pub(crate) mcp: Option<Arc<McpHub>>,
}
