use anyhow::Context;

use crate::records::{ScheduleCreateToolArgs, ScheduleRecord};
use crate::AgentRuntime;

impl AgentRuntime {
    pub(crate) fn schedule_create(&self, args: ScheduleCreateToolArgs) -> anyhow::Result<String> {
        let id = args.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let mut schedules = self.schedules_get()?;
        if let Some(existing) = schedules.iter().find(|schedule| schedule.id == id) {
            return Ok(serde_json::to_string(existing).unwrap_or_else(|_| "ok".to_string()));
        }

        let agent_id = args.agent_id.or(args.agent);
        let record = ScheduleRecord {
            id: id.clone(),
            description: args.description,
            schedule: args.schedule,
            agent_id,
            created_at: Self::now_epoch_seconds(),
            enabled: args.enabled.unwrap_or(true),
        };

        schedules.push(record.clone());
        self.schedules_set(&schedules)?;

        Ok(serde_json::to_string(&record).unwrap_or_else(|_| "ok".to_string()))
    }

    pub(crate) fn schedule_list(&self) -> anyhow::Result<String> {
        let schedules = self.schedules_get()?;
        Ok(serde_json::to_string(&schedules).context("serialize schedule_list")?)
    }

    pub(crate) fn schedule_delete(&self, id: &str) -> anyhow::Result<String> {
        let mut schedules = self.schedules_get()?;
        let before = schedules.len();
        schedules.retain(|schedule| schedule.id != id);
        if schedules.len() == before {
            return Ok(format!("error: schedule not found: {id}"));
        }
        self.schedules_set(&schedules)?;
        Ok("ok".to_string())
    }
}
