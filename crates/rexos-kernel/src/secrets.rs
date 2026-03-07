use crate::config::RexosConfig;
use crate::security::SecretMode;

#[derive(Debug, Clone, Copy, Default)]
pub struct SecretResolver;

impl SecretResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve_env(&self, env_name: &str) -> Option<String> {
        let env_name = env_name.trim();
        if env_name.is_empty() {
            return None;
        }
        std::env::var(env_name).ok()
    }

    pub fn resolve_llm_api_key(&self, cfg: &RexosConfig) -> Option<String> {
        match cfg.security.secrets.mode {
            SecretMode::EnvFirst => self.resolve_env(&cfg.llm.api_key_env),
        }
    }

    pub fn resolve_provider_api_key(&self, cfg: &RexosConfig, provider: &str) -> Option<String> {
        let env_name = cfg
            .providers
            .get(provider)
            .map(|provider| provider.api_key_env.as_str())
            .unwrap_or("");

        match cfg.security.secrets.mode {
            SecretMode::EnvFirst => self.resolve_env(env_name),
        }
    }
}
