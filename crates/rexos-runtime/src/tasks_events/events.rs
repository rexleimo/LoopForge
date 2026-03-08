use anyhow::Context;

use crate::records::{EventPublishToolArgs, EventRecord};
use crate::AgentRuntime;

const EVENTS_KEY: &str = "rexos.events";
const MAX_EVENTS: usize = 200;

impl AgentRuntime {
    pub(crate) fn event_publish(&self, args: EventPublishToolArgs) -> anyhow::Result<String> {
        let raw = self
            .memory
            .kv_get(EVENTS_KEY)
            .context("kv_get rexos.events")?
            .unwrap_or_else(|| "[]".to_string());
        let mut events: Vec<EventRecord> = serde_json::from_str(&raw).unwrap_or_default();

        events.push(EventRecord {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: args.event_type,
            payload: args.payload.unwrap_or(serde_json::json!({})),
            created_at: Self::now_epoch_seconds(),
        });

        if events.len() > MAX_EVENTS {
            events.drain(0..(events.len() - MAX_EVENTS));
        }

        let out = serde_json::to_string(&events).context("serialize rexos.events")?;
        self.memory
            .kv_set(EVENTS_KEY, &out)
            .context("kv_set rexos.events")?;
        Ok("ok".to_string())
    }
}
