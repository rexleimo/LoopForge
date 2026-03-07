#[derive(Debug, serde::Deserialize)]
pub(crate) struct A2aDiscoverArgs {
    pub(crate) url: String,
    #[serde(default)]
    pub(crate) allow_private: bool,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct A2aSendArgs {
    #[serde(default)]
    pub(crate) agent_url: Option<String>,
    #[serde(default)]
    pub(crate) url: Option<String>,
    pub(crate) message: String,
    #[serde(default)]
    pub(crate) session_id: Option<String>,
    #[serde(default)]
    pub(crate) allow_private: bool,
}
