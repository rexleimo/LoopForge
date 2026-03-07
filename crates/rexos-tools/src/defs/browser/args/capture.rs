#[derive(Debug, serde::Deserialize)]
pub(crate) struct BrowserScreenshotArgs {
    #[serde(default)]
    pub(crate) path: Option<String>,
}
