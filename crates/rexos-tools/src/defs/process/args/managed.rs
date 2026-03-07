#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProcessStartArgs {
    pub(crate) command: String,
    #[serde(default)]
    pub(crate) args: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProcessPollArgs {
    pub(crate) process_id: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProcessWriteArgs {
    pub(crate) process_id: String,
    pub(crate) data: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProcessKillArgs {
    pub(crate) process_id: String,
}
