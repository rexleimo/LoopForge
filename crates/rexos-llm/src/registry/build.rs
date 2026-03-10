use std::collections::BTreeMap;
use std::sync::Arc;

use rexos_kernel::config::{ProviderConfig, ProviderKind, RexosConfig};

use crate::anthropic::AnthropicDriver;
#[cfg(feature = "bedrock")]
use crate::bedrock::BedrockDriver;
use crate::dashscope::DashscopeDriver;
#[cfg(not(feature = "bedrock"))]
use crate::driver::UnimplementedDriver;
use crate::driver::{LlmDriver, OpenAiCompatDriver};
use crate::gemini::GeminiDriver;
use crate::minimax::MiniMaxDriver;
use crate::zhipu::ZhipuDriver;

use super::LlmRegistry;

impl LlmRegistry {
    pub fn from_config(cfg: &RexosConfig) -> anyhow::Result<Self> {
        let mut drivers: BTreeMap<String, Arc<dyn LlmDriver>> = BTreeMap::new();
        let mut default_models: BTreeMap<String, String> = BTreeMap::new();

        for (name, provider) in &cfg.providers {
            let driver = build_driver(cfg, name, provider)?;
            drivers.insert(name.clone(), driver);
            default_models.insert(name.clone(), provider.default_model.clone());
        }

        Ok(Self {
            drivers,
            default_models,
        })
    }
}

fn build_driver(
    cfg: &RexosConfig,
    name: &str,
    provider: &ProviderConfig,
) -> anyhow::Result<Arc<dyn LlmDriver>> {
    let api_key = cfg.provider_api_key(name);

    let driver: Arc<dyn LlmDriver> = match provider.kind {
        ProviderKind::OpenAiCompatible => {
            Arc::new(OpenAiCompatDriver::new(provider.base_url.clone(), api_key)?)
        }
        ProviderKind::DashscopeNative => {
            Arc::new(DashscopeDriver::new(provider.base_url.clone(), api_key)?)
        }
        ProviderKind::ZhipuNative => {
            Arc::new(ZhipuDriver::new(provider.base_url.clone(), api_key)?)
        }
        ProviderKind::MiniMaxNative => {
            Arc::new(MiniMaxDriver::new(provider.base_url.clone(), api_key)?)
        }
        ProviderKind::Anthropic => {
            Arc::new(AnthropicDriver::new(provider.base_url.clone(), api_key)?)
        }
        ProviderKind::Gemini => Arc::new(GeminiDriver::new(provider.base_url.clone(), api_key)?),
        ProviderKind::Bedrock => {
            #[cfg(feature = "bedrock")]
            {
                Arc::new(BedrockDriver::new(provider.aws_bedrock.as_ref())?)
            }
            #[cfg(not(feature = "bedrock"))]
            {
                Arc::new(UnimplementedDriver::new("bedrock"))
            }
        }
    };

    Ok(driver)
}
