#[derive(Debug, serde::Deserialize)]
pub(crate) struct BrowserWaitArgs {
    pub(crate) selector: String,
    #[serde(default)]
    pub(crate) timeout_ms: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BrowserWaitForArgs {
    #[serde(default)]
    pub(crate) selector: Option<String>,
    #[serde(default)]
    pub(crate) text: Option<String>,
    #[serde(default)]
    pub(crate) timeout_ms: Option<u64>,
}
