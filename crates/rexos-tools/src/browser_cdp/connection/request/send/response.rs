use std::time::Duration;

use serde_json::Value;
use tokio::sync::oneshot;

use super::super::super::CdpConnection;

pub(super) fn response_channel_closed_error() -> anyhow::Error {
    anyhow::anyhow!("CDP response channel closed")
}

pub(super) fn response_timeout_error() -> anyhow::Error {
    anyhow::anyhow!("CDP command timed out")
}

pub(super) async fn await_response(
    connection: &CdpConnection,
    id: u64,
    rx: oneshot::Receiver<anyhow::Result<Value>>,
) -> anyhow::Result<Value> {
    match tokio::time::timeout(
        Duration::from_secs(super::super::super::super::CDP_COMMAND_TIMEOUT_SECS),
        rx,
    )
    .await
    {
        Ok(Ok(result)) => result,
        Ok(Err(_)) => Err(response_channel_closed_error()),
        Err(_) => {
            connection.pending.remove(&id);
            Err(response_timeout_error())
        }
    }
}
