mod message;
mod response;
#[cfg(test)]
mod tests;

use std::sync::atomic::Ordering;

use serde_json::Value;
use tokio::sync::oneshot;

use super::super::CdpConnection;

impl CdpConnection {
    pub(crate) async fn send(&self, method: &str, params: Value) -> anyhow::Result<Value> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        self.pending.insert(id, tx);

        message::send_request_message(self, id, method, params).await?;
        response::await_response(self, id, rx).await
    }
}
