use anyhow::{bail, Context};

use crate::records::{
    AgentSpawnToolArgs, HandActivateToolArgs, HandInstanceRecord, HandInstanceStatus,
};
use crate::AgentRuntime;

impl AgentRuntime {
    pub(crate) fn hand_list(&self) -> anyhow::Result<String> {
        let defs = Self::hand_defs();
        let index = self.hands_instances_index()?;

        let mut instances = Vec::new();
        for id in index {
            if let Some(record) = self.get_hand_instance(&id)? {
                instances.push(record);
            }
        }

        let out: Vec<serde_json::Value> = defs
            .into_iter()
            .map(|def| {
                let active = instances
                    .iter()
                    .filter(|record| {
                        record.hand_id == def.id && record.status == HandInstanceStatus::Active
                    })
                    .max_by_key(|record| record.created_at);

                serde_json::json!({
                    "id": def.id,
                    "name": def.name,
                    "description": def.description,
                    "status": if active.is_some() { "active" } else { "available" },
                    "instance_id": active.as_ref().map(|record| record.instance_id.clone()),
                    "agent_id": active.as_ref().map(|record| record.agent_id.clone()),
                })
            })
            .collect();

        serde_json::to_string(&out).context("serialize hand_list")
    }

    pub(crate) fn hand_activate(&self, args: HandActivateToolArgs) -> anyhow::Result<String> {
        let hand_id = args.hand_id.trim();
        if hand_id.is_empty() {
            bail!("hand_id is empty");
        }

        let def = Self::hand_defs()
            .into_iter()
            .find(|def| def.id == hand_id)
            .ok_or_else(|| anyhow::anyhow!("unknown hand_id: {hand_id}"))?;

        let instance_id = uuid::Uuid::new_v4().to_string();
        let agent_id = instance_id.clone();

        let mut system_prompt = def.system_prompt.to_string();
        if let Some(config) = args.config.as_ref() {
            system_prompt.push_str("\n\nHand config (JSON):\n");
            system_prompt.push_str(&serde_json::to_string_pretty(config).unwrap_or_default());
        }

        let _ = self.agent_spawn(AgentSpawnToolArgs {
            agent_id: Some(agent_id.clone()),
            name: Some(format!("hand:{hand_id}")),
            system_prompt: Some(system_prompt),
            manifest_toml: None,
        })?;

        let record = HandInstanceRecord {
            instance_id: instance_id.clone(),
            hand_id: hand_id.to_string(),
            agent_id: agent_id.clone(),
            status: HandInstanceStatus::Active,
            created_at: Self::now_epoch_seconds(),
            deactivated_at: None,
            config: args.config.unwrap_or(serde_json::Value::Null),
        };
        self.put_hand_instance(&record)?;

        let mut index = self.hands_instances_index()?;
        if !index.iter().any(|id| id == &instance_id) {
            index.push(instance_id.clone());
            self.put_hands_instances_index(&index)?;
        }

        Ok(serde_json::json!({
            "instance_id": instance_id,
            "hand_id": hand_id,
            "agent_id": agent_id,
            "status": "active",
        })
        .to_string())
    }

    pub(crate) fn hand_status(&self, hand_id: &str) -> anyhow::Result<String> {
        let hand_id = hand_id.trim();
        if hand_id.is_empty() {
            bail!("hand_id is empty");
        }

        let index = self.hands_instances_index()?;
        let mut active: Option<HandInstanceRecord> = None;

        for id in index {
            let Some(record) = self.get_hand_instance(&id)? else {
                continue;
            };
            if record.hand_id != hand_id {
                continue;
            }
            if record.status != HandInstanceStatus::Active {
                continue;
            }

            if active
                .as_ref()
                .map(|current| current.created_at <= record.created_at)
                .unwrap_or(true)
            {
                active = Some(record);
            }
        }

        let Some(active) = active else {
            return Ok(serde_json::json!({
                "hand_id": hand_id,
                "status": "inactive",
            })
            .to_string());
        };

        serde_json::to_string(&active).context("serialize hand_status")
    }

    pub(crate) fn hand_deactivate(&self, instance_id: &str) -> anyhow::Result<String> {
        let instance_id = instance_id.trim();
        if instance_id.is_empty() {
            bail!("instance_id is empty");
        }

        let Some(mut record) = self.get_hand_instance(instance_id)? else {
            return Ok(format!("error: hand instance not found: {instance_id}"));
        };

        if record.status == HandInstanceStatus::Deactivated {
            return Ok("ok".to_string());
        }

        record.status = HandInstanceStatus::Deactivated;
        record.deactivated_at = Some(Self::now_epoch_seconds());
        self.put_hand_instance(&record)?;

        let _ = self.agent_kill(&record.agent_id);
        Ok("ok".to_string())
    }
}
