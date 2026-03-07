#[derive(Debug, serde::Deserialize)]
pub(crate) struct ShellArgs {
    pub(crate) command: String,
    #[serde(default)]
    pub(crate) timeout_ms: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ShellExecArgs {
    pub(crate) command: String,
    #[serde(default)]
    pub(crate) timeout_seconds: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct DockerExecArgs {
    pub(crate) command: String,
}
