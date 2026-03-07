use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use dashmap::DashMap;
use serde_json::Value;
use tokio::sync::{oneshot, Mutex};
use tokio_tungstenite::tungstenite::Message as WsMessage;

mod reader;
mod request;

type WsStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

pub(super) struct CdpConnection {
    write: Arc<Mutex<futures::stream::SplitSink<WsStream, WsMessage>>>,
    pending: Arc<DashMap<u64, oneshot::Sender<anyhow::Result<Value>>>>,
    next_id: AtomicU64,
    reader_handle: tokio::task::JoinHandle<()>,
}

impl Drop for CdpConnection {
    fn drop(&mut self) {
        self.reader_handle.abort();
    }
}
