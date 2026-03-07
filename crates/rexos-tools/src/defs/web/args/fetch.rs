#[derive(Debug, serde::Deserialize)]
pub(crate) struct WebFetchArgs {
    pub(crate) url: String,
    #[serde(default)]
    pub(crate) timeout_ms: Option<u64>,
    #[serde(default)]
    pub(crate) max_bytes: Option<u64>,
    #[serde(default)]
    pub(crate) allow_private: bool,
}
