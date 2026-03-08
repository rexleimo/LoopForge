use anyhow::Context;

use super::delivery::{deliver_console, deliver_webhook};
use super::events::{record_delivery_failed, record_delivery_sent};
use super::store::{outbox_messages_get, outbox_messages_set};
use super::{OutboxDispatcher, OutboxDrainSummary};
use crate::records::OutboxStatus;
use crate::AgentRuntime;

impl OutboxDispatcher {
    pub fn new(memory: rexos_memory::MemoryStore) -> anyhow::Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .context("build http client")?;
        Ok(Self { memory, http })
    }

    pub async fn drain_once(&self, limit: usize) -> anyhow::Result<OutboxDrainSummary> {
        let mut msgs = outbox_messages_get(&self.memory)?;
        let mut summary = OutboxDrainSummary::default();
        let mut processed = 0usize;

        for msg in msgs.iter_mut() {
            if processed >= limit.max(1) {
                break;
            }
            if msg.status != OutboxStatus::Queued {
                continue;
            }
            processed += 1;

            let now = AgentRuntime::now_epoch_seconds();
            msg.attempts = msg.attempts.saturating_add(1);
            msg.updated_at = now;
            msg.last_error = None;

            let result = match msg.channel.as_str() {
                "console" => {
                    deliver_console(msg);
                    Ok(())
                }
                "webhook" => deliver_webhook(&self.http, msg).await,
                other => Err(anyhow::anyhow!("unknown channel: {other}")),
            };

            match result {
                Ok(()) => {
                    msg.status = OutboxStatus::Sent;
                    msg.sent_at = Some(now);
                    summary.sent = summary.sent.saturating_add(1);
                    record_delivery_sent(&self.memory, msg, now);
                }
                Err(error) => {
                    msg.status = OutboxStatus::Failed;
                    msg.last_error = Some(error.to_string());
                    summary.failed = summary.failed.saturating_add(1);
                    record_delivery_failed(&self.memory, msg, now);
                }
            }
        }

        if processed > 0 {
            outbox_messages_set(&self.memory, &msgs)?;
        }

        Ok(summary)
    }
}
