#[derive(Debug, serde::Deserialize)]
pub(crate) struct MemoryStoreToolArgs {
    pub(crate) key: String,
    pub(crate) value: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MemoryRecallToolArgs {
    pub(crate) key: String,
}
