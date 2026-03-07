use std::sync::Arc;

use dashmap::DashMap;
use futures::StreamExt;
use serde_json::Value;
use tokio::sync::oneshot;

use super::super::WsStream;

impl super::super::CdpConnection {
    pub(crate) async fn reader_loop(
        mut read: futures::stream::SplitStream<WsStream>,
        pending: Arc<DashMap<u64, oneshot::Sender<anyhow::Result<Value>>>>,
    ) {
        while let Some(msg) = read.next().await {
            let Some(text) = super::incoming_message_text(msg) else {
                break;
            };
            if text.is_empty() {
                continue;
            }

            let json: Value = match serde_json::from_str(&text) {
                Ok(value) => value,
                Err(_) => continue,
            };

            if let Some(id) = super::response_id(&json) {
                if let Some((_, sender)) = pending.remove(&id) {
                    if let Some(error) = super::parsed_response_error(&json) {
                        let _ = sender.send(Err(error));
                    } else {
                        let _ = sender.send(Ok(super::response_result(&json)));
                    }
                }
            }
        }
    }
}
