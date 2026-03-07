use std::collections::BTreeMap;
use std::sync::Arc;

use rexos_kernel::config::{ProviderKind, RexosConfig};

use crate::anthropic::AnthropicDriver;
use crate::dashscope::DashscopeDriver;
use crate::driver::{LlmDriver, OpenAiCompatDriver};
use crate::gemini::GeminiDriver;
use crate::minimax::MiniMaxDriver;
use crate::zhipu::ZhipuDriver;

#[derive(Clone)]
pub struct LlmRegistry {
    drivers: BTreeMap<String, Arc<dyn LlmDriver>>,
    default_models: BTreeMap<String, String>,
}

impl std::fmt::Debug for LlmRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let keys: Vec<&str> = self.drivers.keys().map(|k| k.as_str()).collect();
        f.debug_struct("LlmRegistry")
            .field("providers", &keys)
            .finish()
    }
}

impl LlmRegistry {
    pub fn from_config(cfg: &RexosConfig) -> anyhow::Result<Self> {
        let mut drivers: BTreeMap<String, Arc<dyn LlmDriver>> = BTreeMap::new();
        let mut default_models: BTreeMap<String, String> = BTreeMap::new();

        for (name, p) in &cfg.providers {
            let driver: Arc<dyn LlmDriver> = match p.kind {
                ProviderKind::OpenAiCompatible => Arc::new(OpenAiCompatDriver::new(
                    p.base_url.clone(),
                    cfg.provider_api_key(name),
                )?),
                ProviderKind::DashscopeNative => Arc::new(DashscopeDriver::new(
                    p.base_url.clone(),
                    cfg.provider_api_key(name),
                )?),
                ProviderKind::ZhipuNative => Arc::new(ZhipuDriver::new(
                    p.base_url.clone(),
                    cfg.provider_api_key(name),
                )?),
                ProviderKind::MiniMaxNative => Arc::new(MiniMaxDriver::new(
                    p.base_url.clone(),
                    cfg.provider_api_key(name),
                )?),
                ProviderKind::Anthropic => Arc::new(AnthropicDriver::new(
                    p.base_url.clone(),
                    cfg.provider_api_key(name),
                )?),
                ProviderKind::Gemini => Arc::new(GeminiDriver::new(
                    p.base_url.clone(),
                    cfg.provider_api_key(name),
                )?),
            };

            drivers.insert(name.clone(), driver);
            default_models.insert(name.clone(), p.default_model.clone());
        }

        Ok(Self {
            drivers,
            default_models,
        })
    }

    pub fn driver(&self, name: &str) -> Option<Arc<dyn LlmDriver>> {
        self.drivers.get(name).cloned()
    }

    pub fn default_model(&self, provider: &str) -> Option<&str> {
        self.default_models
            .get(provider)
            .map(|m| m.as_str())
            .filter(|m| !m.trim().is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rexos_kernel::config::{
        LlmConfig, ProviderConfig, ProviderKind, RexosConfig, RouterConfig,
    };

    #[test]
    fn registry_builds_and_resolves_drivers() {
        let mut providers = BTreeMap::new();
        providers.insert(
            "ollama".to_string(),
            ProviderConfig {
                kind: ProviderKind::OpenAiCompatible,
                base_url: "http://127.0.0.1:11434/v1".to_string(),
                api_key_env: "".to_string(),
                default_model: "llama3.2".to_string(),
            },
        );
        providers.insert(
            "qwen_native".to_string(),
            ProviderConfig {
                kind: ProviderKind::DashscopeNative,
                base_url: "http://127.0.0.1:1/api/v1".to_string(),
                api_key_env: "DASHSCOPE_API_KEY".to_string(),
                default_model: "qwen-plus".to_string(),
            },
        );
        providers.insert(
            "glm_native".to_string(),
            ProviderConfig {
                kind: ProviderKind::ZhipuNative,
                base_url: "http://127.0.0.1:1/api/paas/v4".to_string(),
                api_key_env: "ZHIPUAI_API_KEY".to_string(),
                default_model: "glm-4".to_string(),
            },
        );
        providers.insert(
            "minimax_native".to_string(),
            ProviderConfig {
                kind: ProviderKind::MiniMaxNative,
                base_url: "http://127.0.0.1:1/v1".to_string(),
                api_key_env: "MINIMAX_API_KEY".to_string(),
                default_model: "MiniMax-M2.5".to_string(),
            },
        );
        providers.insert(
            "anthropic".to_string(),
            ProviderConfig {
                kind: ProviderKind::Anthropic,
                base_url: "http://127.0.0.1:1".to_string(),
                api_key_env: "ANTHROPIC_API_KEY".to_string(),
                default_model: "claude-test".to_string(),
            },
        );

        let cfg = RexosConfig {
            llm: LlmConfig::default(),
            providers,
            router: RouterConfig::default(),
        };

        let registry = LlmRegistry::from_config(&cfg).unwrap();
        assert!(registry.driver("ollama").is_some());
        assert!(registry.driver("qwen_native").is_some());
        assert!(registry.driver("glm_native").is_some());
        assert!(registry.driver("minimax_native").is_some());
        assert!(registry.driver("anthropic").is_some());
    }
}
