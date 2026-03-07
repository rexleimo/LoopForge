use anyhow::Context;
use futures::SinkExt;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message as WsMessage;

use super::super::super::CdpConnection;

pub(super) fn request_message_text(id: u64, method: &str, params: Value) -> String {
    serde_json::json!({ "id": id, "method": method, "params": params }).to_string()
}

pub(super) async fn send_request_message(
    connection: &CdpConnection,
    id: u64,
    method: &str,
    params: Value,
) -> anyhow::Result<()> {
    connection
        .write
        .lock()
        .await
        .send(WsMessage::Text(
            request_message_text(id, method, params).into(),
        ))
        .await
        .context("CDP send")
}
