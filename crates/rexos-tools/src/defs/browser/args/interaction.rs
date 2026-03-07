#[derive(Debug, serde::Deserialize)]
pub(crate) struct BrowserRunJsArgs {
    pub(crate) expression: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BrowserClickArgs {
    pub(crate) selector: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BrowserTypeArgs {
    pub(crate) selector: String,
    pub(crate) text: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BrowserPressKeyArgs {
    pub(crate) key: String,
    #[serde(default)]
    pub(crate) selector: Option<String>,
}
