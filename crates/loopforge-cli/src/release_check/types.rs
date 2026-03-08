#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ReleaseCheckItem {
    pub(crate) id: String,
    pub(crate) ok: bool,
    pub(crate) message: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ReleaseCheckReport {
    pub(crate) ok: bool,
    pub(crate) tag: String,
    pub(crate) checks: Vec<ReleaseCheckItem>,
}
