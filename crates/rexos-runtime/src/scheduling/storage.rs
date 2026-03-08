use anyhow::Context;

use crate::records::{CronJobRecord, ScheduleRecord};
use crate::AgentRuntime;

impl AgentRuntime {
    pub(super) fn schedules_get(&self) -> anyhow::Result<Vec<ScheduleRecord>> {
        let key = "rexos.schedules";
        let raw = self
            .memory
            .kv_get(key)
            .context("kv_get rexos.schedules")?
            .unwrap_or_else(|| "[]".to_string());
        let schedules: Vec<ScheduleRecord> = serde_json::from_str(&raw).unwrap_or_default();
        Ok(schedules)
    }

    pub(super) fn schedules_set(&self, schedules: &[ScheduleRecord]) -> anyhow::Result<()> {
        let key = "rexos.schedules";
        let raw = serde_json::to_string(schedules).context("serialize rexos.schedules")?;
        self.memory
            .kv_set(key, &raw)
            .context("kv_set rexos.schedules")?;
        Ok(())
    }

    pub(super) fn cron_jobs_get(&self) -> anyhow::Result<Vec<CronJobRecord>> {
        let key = "rexos.cron.jobs";
        let raw = self
            .memory
            .kv_get(key)
            .context("kv_get rexos.cron.jobs")?
            .unwrap_or_else(|| "[]".to_string());
        let jobs: Vec<CronJobRecord> = serde_json::from_str(&raw).unwrap_or_default();
        Ok(jobs)
    }

    pub(super) fn cron_jobs_set(&self, jobs: &[CronJobRecord]) -> anyhow::Result<()> {
        let key = "rexos.cron.jobs";
        let raw = serde_json::to_string(jobs).context("serialize rexos.cron.jobs")?;
        self.memory
            .kv_set(key, &raw)
            .context("kv_set rexos.cron.jobs")?;
        Ok(())
    }
}
