use anyhow::Context;

use crate::records::{TaskPostToolArgs, TaskRecord, TaskStatus};
use crate::AgentRuntime;

impl AgentRuntime {
    fn tasks_index(&self) -> anyhow::Result<Vec<String>> {
        let raw = self
            .memory
            .kv_get("rexos.tasks.index")
            .context("kv_get rexos.tasks.index")?
            .unwrap_or_else(|| "[]".to_string());
        let ids: Vec<String> = serde_json::from_str(&raw).unwrap_or_default();
        Ok(ids)
    }

    fn put_tasks_index(&self, ids: &[String]) -> anyhow::Result<()> {
        let raw = serde_json::to_string(ids).context("serialize tasks index")?;
        self.memory
            .kv_set("rexos.tasks.index", &raw)
            .context("kv_set rexos.tasks.index")?;
        Ok(())
    }

    fn task_key(task_id: &str) -> String {
        format!("rexos.tasks.{task_id}")
    }

    fn get_task(&self, task_id: &str) -> anyhow::Result<Option<TaskRecord>> {
        let raw = self
            .memory
            .kv_get(&Self::task_key(task_id))
            .with_context(|| format!("kv_get task {task_id}"))?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let record: TaskRecord =
            serde_json::from_str(&raw).with_context(|| format!("parse task {task_id}"))?;
        Ok(Some(record))
    }

    fn put_task(&self, record: &TaskRecord) -> anyhow::Result<()> {
        let raw = serde_json::to_string(record).context("serialize task record")?;
        self.memory
            .kv_set(&Self::task_key(&record.id), &raw)
            .with_context(|| format!("kv_set task {}", record.id))?;
        Ok(())
    }

    pub(crate) fn task_post(&self, args: TaskPostToolArgs) -> anyhow::Result<String> {
        let task_id = args
            .task_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if let Some(existing) = self.get_task(&task_id)? {
            return Ok(serde_json::to_string(&existing).unwrap_or_else(|_| "ok".to_string()));
        }

        let record = TaskRecord {
            id: task_id.clone(),
            title: args.title,
            description: args.description,
            assigned_to: args.assigned_to,
            status: TaskStatus::Pending,
            claimed_by: None,
            result: None,
            created_at: Self::now_epoch_seconds(),
            claimed_at: None,
            completed_at: None,
        };

        self.put_task(&record)?;
        let mut index = self.tasks_index()?;
        if !index.iter().any(|id| id == &task_id) {
            index.push(task_id);
            self.put_tasks_index(&index)?;
        }

        Ok(serde_json::to_string(&record).unwrap_or_else(|_| "ok".to_string()))
    }

    pub(crate) fn task_list(&self, status: Option<&str>) -> anyhow::Result<String> {
        let wanted = status
            .map(|status| status.trim().to_lowercase())
            .filter(|status| !status.is_empty());

        let index = self.tasks_index()?;
        let mut out = Vec::new();
        for id in index {
            let Some(record) = self.get_task(&id)? else {
                continue;
            };
            if let Some(wanted) = wanted.as_deref() {
                if record.status.as_str() != wanted {
                    continue;
                }
            }
            out.push(record);
        }

        serde_json::to_string(&out).context("serialize task_list")
    }

    pub(crate) fn task_claim(&self, agent_id: Option<&str>) -> anyhow::Result<String> {
        let agent_id = agent_id.map(|id| id.trim()).filter(|id| !id.is_empty());

        let index = self.tasks_index()?;
        for id in index {
            let Some(mut record) = self.get_task(&id)? else {
                continue;
            };
            if record.status != TaskStatus::Pending {
                continue;
            }
            if let Some(assigned) = record.assigned_to.as_deref() {
                let Some(agent_id) = agent_id else {
                    continue;
                };
                if assigned != agent_id {
                    continue;
                }
            }

            record.status = TaskStatus::Claimed;
            record.claimed_by = agent_id.map(|id| id.to_string());
            record.claimed_at = Some(Self::now_epoch_seconds());
            self.put_task(&record)?;
            return Ok(serde_json::to_string(&record).unwrap_or_else(|_| "ok".to_string()));
        }

        Ok("null".to_string())
    }

    pub(crate) fn task_complete(&self, task_id: &str, result: &str) -> anyhow::Result<String> {
        let Some(mut record) = self.get_task(task_id)? else {
            return Ok(format!("error: task not found: {task_id}"));
        };
        record.status = TaskStatus::Completed;
        record.result = Some(result.to_string());
        record.completed_at = Some(Self::now_epoch_seconds());
        self.put_task(&record)?;
        Ok("ok".to_string())
    }
}
