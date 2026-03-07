#[derive(Debug, serde::Deserialize)]
pub(crate) struct WebSearchArgs {
    pub(crate) query: String,
    #[serde(default)]
    pub(crate) max_results: Option<u32>,
}
