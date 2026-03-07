use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;
use serde_json::Value;
use tokio::sync::oneshot;

use super::super::super::WsStream;

pub(super) async fn connect_stream(ws_url: &str) -> anyhow::Result<WsStream> {
    let (stream, _) = tokio::time::timeout(
        Duration::from_secs(super::super::super::super::CDP_CONNECT_TIMEOUT_SECS),
        tokio_tungstenite::connect_async(ws_url),
    )
    .await
    .map_err(|_| anyhow::anyhow!(super::error::timeout_error_message(ws_url)))?
    .map_err(|error| anyhow::anyhow!(super::error::connect_error_message(error)))?;

    Ok(stream)
}

pub(super) fn pending_map() -> Arc<DashMap<u64, oneshot::Sender<anyhow::Result<Value>>>> {
    Arc::new(DashMap::new())
}
