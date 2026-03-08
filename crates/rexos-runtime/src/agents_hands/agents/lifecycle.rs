use std::path::PathBuf;

use crate::records::{AgentRecord, AgentSendToolArgs, AgentSpawnToolArgs, AgentStatus};
use crate::{AgentRuntime, AGENT_CALL_DEPTH, MAX_AGENT_CALL_DEPTH};
use anyhow::Context;
use rexos_kernel::router::TaskKind;

impl AgentRuntime {
    pub(crate) fn agent_spawn(&self, args: AgentSpawnToolArgs) -> anyhow::Result<String> {
        let mut name = args.name;
        let mut system_prompt = args.system_prompt;

        if let Some(manifest_toml) = args.manifest_toml.as_deref() {
            if let Ok(value) = manifest_toml.parse::<toml::Value>() {
                if name.is_none() {
                    name = value
                        .get("name")
                        .and_then(|value| value.as_str())
                        .map(|value| value.to_string());
                }
                if system_prompt.is_none() {
                    system_prompt = value
                        .get("model")
                        .and_then(|model| model.get("system_prompt"))
                        .and_then(|value| value.as_str())
                        .map(|value| value.to_string());
                }
            }
        }

        let agent_id = args
            .agent_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if let Some(existing) = self.get_agent(&agent_id)? {
            return Ok(serde_json::to_string(&existing).unwrap_or_else(|_| "ok".to_string()));
        }

        let record = AgentRecord {
            id: agent_id.clone(),
            name,
            system_prompt,
            status: AgentStatus::Running,
            created_at: Self::now_epoch_seconds(),
            killed_at: None,
        };

        self.put_agent(&record)?;

        let mut index = self.agents_index()?;
        if !index.iter().any(|id| id == &agent_id) {
            index.push(agent_id);
            self.put_agents_index(&index)?;
        }

        Ok(serde_json::to_string(&record).unwrap_or_else(|_| "ok".to_string()))
    }

    pub(crate) fn agent_list(&self) -> anyhow::Result<String> {
        let index = self.agents_index()?;
        let mut out = Vec::new();
        for id in index {
            if let Some(record) = self.get_agent(&id)? {
                out.push(record);
            }
        }
        Ok(serde_json::to_string(&out).context("serialize agent list")?)
    }

    pub(crate) fn agent_find(&self, query: &str) -> anyhow::Result<String> {
        let q = query.trim().to_lowercase();
        if q.is_empty() {
            return Ok("[]".to_string());
        }

        let index = self.agents_index()?;
        let mut out = Vec::new();
        for id in index {
            let Some(record) = self.get_agent(&id)? else {
                continue;
            };
            let hay =
                format!("{} {}", record.id, record.name.clone().unwrap_or_default()).to_lowercase();
            if hay.contains(&q) {
                out.push(record);
            }
        }

        Ok(serde_json::to_string(&out).context("serialize agent find")?)
    }

    pub(crate) fn agent_kill(&self, agent_id: &str) -> anyhow::Result<String> {
        let Some(mut record) = self.get_agent(agent_id)? else {
            return Ok(format!("error: agent not found: {agent_id}"));
        };
        record.status = AgentStatus::Killed;
        record.killed_at = Some(Self::now_epoch_seconds());
        self.put_agent(&record)?;
        Ok("ok".to_string())
    }

    pub(crate) async fn agent_send(
        &self,
        workspace_root: PathBuf,
        kind: TaskKind,
        args: AgentSendToolArgs,
    ) -> anyhow::Result<String> {
        let Some(record) = self.get_agent(&args.agent_id)? else {
            return Ok(format!("error: agent not found: {}", args.agent_id));
        };
        if record.status == AgentStatus::Killed {
            return Ok(format!("error: agent is killed: {}", args.agent_id));
        }

        let current_depth = AGENT_CALL_DEPTH.try_with(|depth| depth.get()).unwrap_or(0);
        if current_depth >= MAX_AGENT_CALL_DEPTH {
            return Ok(format!(
                "error: agent call depth exceeded (max {MAX_AGENT_CALL_DEPTH})"
            ));
        }

        let agent_id = args.agent_id.clone();
        let message = args.message.clone();
        let system_prompt = record.system_prompt.clone();

        let out = AGENT_CALL_DEPTH
            .scope(std::cell::Cell::new(current_depth + 1), async {
                Box::pin(self.run_session(
                    workspace_root,
                    &agent_id,
                    system_prompt.as_deref(),
                    &message,
                    kind,
                ))
                .await
            })
            .await;

        match out {
            Ok(value) => Ok(value),
            Err(err) => Ok(format!("error: {err}")),
        }
    }
}
