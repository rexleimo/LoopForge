use std::collections::HashSet;

use anyhow::Context;

use crate::{
    AgentRuntime, SessionSkillPolicy, SESSION_ALLOWED_SKILLS_KEY_PREFIX,
    SESSION_ALLOWED_TOOLS_KEY_PREFIX, SESSION_MCP_CONFIG_KEY_PREFIX,
    SESSION_SKILL_POLICY_KEY_PREFIX,
};

fn normalize_names(values: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut cleaned = Vec::new();
    let mut seen = HashSet::new();
    for value in values {
        let value = value.trim().to_string();
        if value.is_empty() {
            continue;
        }
        if seen.insert(value.clone()) {
            cleaned.push(value);
        }
    }
    cleaned
}

impl AgentRuntime {
    fn session_allowed_tools_key(session_id: &str) -> String {
        format!("{SESSION_ALLOWED_TOOLS_KEY_PREFIX}{session_id}")
    }

    fn session_mcp_config_key(session_id: &str) -> String {
        format!("{SESSION_MCP_CONFIG_KEY_PREFIX}{session_id}")
    }

    fn session_allowed_skills_key(session_id: &str) -> String {
        format!("{SESSION_ALLOWED_SKILLS_KEY_PREFIX}{session_id}")
    }

    fn session_skill_policy_key(session_id: &str) -> String {
        format!("{SESSION_SKILL_POLICY_KEY_PREFIX}{session_id}")
    }

    pub fn set_session_allowed_tools(
        &self,
        session_id: &str,
        tools: Vec<String>,
    ) -> anyhow::Result<()> {
        let raw = serde_json::to_string(&normalize_names(tools))
            .context("serialize session allowed tools")?;
        self.memory
            .kv_set(&Self::session_allowed_tools_key(session_id), &raw)
            .context("kv_set session allowed tools")?;
        Ok(())
    }

    pub fn set_session_mcp_config(&self, session_id: &str, raw_json: String) -> anyhow::Result<()> {
        let raw_json = raw_json.trim().to_string();
        self.memory
            .kv_set(&Self::session_mcp_config_key(session_id), &raw_json)
            .context("kv_set session mcp config")?;
        Ok(())
    }

    pub(crate) fn load_session_mcp_config(
        &self,
        session_id: &str,
    ) -> anyhow::Result<Option<String>> {
        let raw = self
            .memory
            .kv_get(&Self::session_mcp_config_key(session_id))
            .context("kv_get session mcp config")?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let raw = raw.trim().to_string();
        if raw.is_empty() {
            return Ok(None);
        }
        Ok(Some(raw))
    }

    pub(crate) fn load_session_allowed_tools(
        &self,
        session_id: &str,
    ) -> anyhow::Result<Option<Vec<String>>> {
        let raw = self
            .memory
            .kv_get(&Self::session_allowed_tools_key(session_id))
            .context("kv_get session allowed tools")?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let parsed: Vec<String> = serde_json::from_str(&raw).unwrap_or_default();
        let cleaned = normalize_names(parsed);
        if cleaned.is_empty() {
            return Ok(None);
        }
        Ok(Some(cleaned))
    }

    pub fn set_session_allowed_skills(
        &self,
        session_id: &str,
        skills: Vec<String>,
    ) -> anyhow::Result<()> {
        let raw = serde_json::to_string(&normalize_names(skills))
            .context("serialize session allowed skills")?;
        self.memory
            .kv_set(&Self::session_allowed_skills_key(session_id), &raw)
            .context("kv_set session allowed skills")?;
        Ok(())
    }

    pub(crate) fn load_session_allowed_skills(
        &self,
        session_id: &str,
    ) -> anyhow::Result<Option<Vec<String>>> {
        let raw = self
            .memory
            .kv_get(&Self::session_allowed_skills_key(session_id))
            .context("kv_get session allowed skills")?;
        let Some(raw) = raw else {
            return Ok(None);
        };
        let parsed: Vec<String> = serde_json::from_str(&raw).unwrap_or_default();
        let cleaned = normalize_names(parsed);
        if cleaned.is_empty() {
            return Ok(None);
        }
        Ok(Some(cleaned))
    }

    pub fn set_session_skill_policy(
        &self,
        session_id: &str,
        policy: SessionSkillPolicy,
    ) -> anyhow::Result<()> {
        let raw = serde_json::to_string(&policy).context("serialize session skill policy")?;
        self.memory
            .kv_set(&Self::session_skill_policy_key(session_id), &raw)
            .context("kv_set session skill policy")?;
        Ok(())
    }

    pub(crate) fn load_session_skill_policy(
        &self,
        session_id: &str,
    ) -> anyhow::Result<SessionSkillPolicy> {
        let raw = self
            .memory
            .kv_get(&Self::session_skill_policy_key(session_id))
            .context("kv_get session skill policy")?;
        let Some(raw) = raw else {
            return Ok(SessionSkillPolicy::default());
        };
        let policy: SessionSkillPolicy = serde_json::from_str(&raw).unwrap_or_default();
        Ok(policy)
    }
}
