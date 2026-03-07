#[derive(Debug, serde::Deserialize)]
pub(crate) struct PdfArgs {
    pub(crate) path: String,
    #[serde(default)]
    pub(crate) pages: Option<String>,
    #[serde(default)]
    pub(crate) max_pages: Option<u64>,
    #[serde(default)]
    pub(crate) max_chars: Option<u64>,
}
