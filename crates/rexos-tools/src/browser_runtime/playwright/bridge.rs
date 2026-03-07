use anyhow::bail;

#[derive(Clone, Debug, serde::Deserialize)]
pub(crate) struct BridgeResponse {
    success: bool,
    #[serde(default)]
    data: Option<serde_json::Value>,
    #[serde(default)]
    error: Option<String>,
}

impl BridgeResponse {
    pub(crate) fn into_data(self) -> anyhow::Result<serde_json::Value> {
        if self.success {
            return Ok(self
                .data
                .unwrap_or_else(|| serde_json::json!({ "status": "ok" })));
        }
        bail!(
            "browser bridge error: {}",
            self.error.unwrap_or_else(|| "unknown error".to_string())
        )
    }
}
