use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ToolsListResult {
    #[serde(default)]
    pub(crate) tools: Vec<McpTool>,
    #[serde(default, rename = "nextCursor")]
    pub(crate) next_cursor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct McpTool {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) description: Option<String>,
    #[serde(default, rename = "inputSchema")]
    pub(crate) input_schema: serde_json::Value,
}
