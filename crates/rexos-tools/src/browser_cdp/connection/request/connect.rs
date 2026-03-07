mod error;
mod stream;
#[cfg(test)]
mod tests;

use std::sync::Arc;

use futures::StreamExt;
use tokio::sync::Mutex;

use super::super::CdpConnection;

impl CdpConnection {
    pub(crate) async fn connect(ws_url: &str) -> anyhow::Result<Self> {
        let stream = stream::connect_stream(ws_url).await?;
        let (write, read) = stream.split();
        let write = Arc::new(Mutex::new(write));
        let pending = stream::pending_map();

        let reader_handle = tokio::spawn(Self::reader_loop(read, Arc::clone(&pending)));

        Ok(Self {
            write,
            pending,
            next_id: std::sync::atomic::AtomicU64::new(1),
            reader_handle,
        })
    }
}
