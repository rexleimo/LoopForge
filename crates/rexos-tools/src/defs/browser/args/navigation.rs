#[derive(Debug, serde::Deserialize)]
pub(crate) struct BrowserNavigateArgs {
    pub(crate) url: String,
    #[serde(default)]
    pub(crate) timeout_ms: Option<u64>,
    #[serde(default)]
    pub(crate) allow_private: bool,
    #[serde(default)]
    pub(crate) headless: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BrowserScrollArgs {
    #[serde(default)]
    pub(crate) direction: Option<String>,
    #[serde(default)]
    pub(crate) amount: Option<i64>,
}
