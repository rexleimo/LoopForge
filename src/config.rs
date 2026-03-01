use std::fs;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::paths::RexosPaths;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RexosConfig {
    pub llm: LlmConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LlmConfig {
    /// Base URL for OpenAI-compatible API (example: https://api.openai.com/v1)
    pub base_url: String,
    /// Environment variable name holding the API key.
    pub api_key_env: String,
    /// Default model name.
    pub model: String,
}

impl Default for RexosConfig {
    fn default() -> Self {
        Self {
            llm: LlmConfig::default(),
        }
    }
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key_env: "OPENAI_API_KEY".to_string(),
            model: "gpt-4.1-mini".to_string(),
        }
    }
}

impl RexosConfig {
    pub fn ensure_default(paths: &RexosPaths) -> anyhow::Result<()> {
        let config_path = paths.config_path();
        if config_path.exists() {
            return Ok(());
        }

        let default_config = RexosConfig::default();
        let toml_str = toml::to_string_pretty(&default_config).context("serialize config")?;

        fs::write(&config_path, toml_str)
            .with_context(|| format!("write config: {}", config_path.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_serializes() {
        let cfg = RexosConfig::default();
        let toml_str = toml::to_string_pretty(&cfg).unwrap();
        assert!(toml_str.contains("base_url"));
        assert!(toml_str.contains("api_key_env"));
        assert!(toml_str.contains("model"));
    }
}

