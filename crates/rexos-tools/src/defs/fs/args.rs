#[derive(Debug, serde::Deserialize)]
pub(crate) struct FsReadArgs {
    pub(crate) path: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct FileReadArgs {
    pub(crate) path: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct FsWriteArgs {
    pub(crate) path: String,
    pub(crate) content: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct FileWriteArgs {
    pub(crate) path: String,
    pub(crate) content: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct FileListArgs {
    pub(crate) path: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ApplyPatchArgs {
    pub(crate) patch: String,
}
