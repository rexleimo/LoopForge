use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::{oneshot, Mutex};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

pub(crate) struct JsonRpcClient {
    next_id: AtomicU64,
    writer: Arc<Mutex<BufWriter<Box<dyn tokio::io::AsyncWrite + Unpin + Send>>>>,
    pending: Arc<Mutex<HashMap<u64, oneshot::Sender<anyhow::Result<Value>>>>>,
    _reader_task: tokio::task::JoinHandle<()>,
}

impl std::fmt::Debug for JsonRpcClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonRpcClient").finish_non_exhaustive()
    }
}

impl JsonRpcClient {
    pub(crate) fn new<R, W>(reader: R, writer: W) -> Self
    where
        R: tokio::io::AsyncRead + Unpin + Send + 'static,
        W: tokio::io::AsyncWrite + Unpin + Send + 'static,
    {
        let writer: Box<dyn tokio::io::AsyncWrite + Unpin + Send> = Box::new(writer);
        let writer = Arc::new(Mutex::new(BufWriter::new(writer)));
        let pending: Arc<Mutex<HashMap<u64, oneshot::Sender<anyhow::Result<Value>>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let pending_reader = pending.clone();

        let mut lines = BufReader::new(reader).lines();
        let reader_task = tokio::spawn(async move {
            while let Ok(Some(line)) = lines.next_line().await {
                if line.trim().is_empty() {
                    continue;
                }
                let value: Value = match serde_json::from_str(&line) {
                    Ok(value) => value,
                    Err(_) => continue,
                };

                let id = value.get("id").and_then(|v| v.as_u64());
                if let Some(id) = id {
                    let result = if let Some(err) = value.get("error") {
                        Err(anyhow!("jsonrpc error: {}", err))
                    } else {
                        Ok(value.get("result").cloned().unwrap_or(Value::Null))
                    };
                    let tx = pending_reader.lock().await.remove(&id);
                    if let Some(tx) = tx {
                        let _ = tx.send(result);
                    }
                }
            }
        });

        Self {
            next_id: AtomicU64::new(1),
            writer,
            pending,
            _reader_task: reader_task,
        }
    }

    pub(crate) async fn notify(&self, method: &str, params: Option<Value>) -> anyhow::Result<()> {
        let mut msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
        });
        if let Some(params) = params {
            msg["params"] = params;
        }
        self.send_message(&msg).await
    }

    pub(crate) async fn request(
        &self,
        method: &str,
        params: Option<Value>,
    ) -> anyhow::Result<Value> {
        self.request_with_timeout(method, params, DEFAULT_TIMEOUT)
            .await
    }

    pub(crate) async fn request_with_timeout(
        &self,
        method: &str,
        params: Option<Value>,
        timeout: Duration,
    ) -> anyhow::Result<Value> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let mut msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
        });
        if let Some(params) = params {
            msg["params"] = params;
        }

        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);
        if let Err(err) = self.send_message(&msg).await {
            self.pending.lock().await.remove(&id);
            return Err(err).with_context(|| format!("send jsonrpc request: {method}"));
        }

        let result = match tokio::time::timeout(timeout, rx).await {
            Ok(result) => result,
            Err(_) => {
                self.pending.lock().await.remove(&id);
                return Err(anyhow!("jsonrpc timeout waiting for response: {method}"));
            }
        };

        let result = match result {
            Ok(result) => result,
            Err(_) => {
                self.pending.lock().await.remove(&id);
                return Err(anyhow!("jsonrpc response channel closed: {method}"));
            }
        }?;
        Ok(result)
    }

    async fn send_message(&self, msg: &Value) -> anyhow::Result<()> {
        let payload = serde_json::to_string(msg).context("serialize jsonrpc message")?;
        if payload.contains('\n') {
            return Err(anyhow!(
                "jsonrpc message contains newline (invalid for stdio framing)"
            ));
        }

        let mut writer = self.writer.lock().await;
        writer.write_all(payload.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
        Ok(())
    }
}
