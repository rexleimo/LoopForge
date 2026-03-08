use std::collections::BTreeMap;

use super::{LlmConfig, ProviderConfig, ProviderKind, RouteConfig, RouterConfig, SkillsConfig};

fn provider(
    kind: ProviderKind,
    base_url: &str,
    api_key_env: &str,
    default_model: &str,
) -> ProviderConfig {
    ProviderConfig {
        kind,
        base_url: base_url.to_string(),
        api_key_env: api_key_env.to_string(),
        default_model: default_model.to_string(),
    }
}

pub(super) fn default_providers() -> BTreeMap<String, ProviderConfig> {
    let mut providers = BTreeMap::new();
    providers.insert(
        "ollama".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "http://127.0.0.1:11434/v1",
            "",
            "llama3.2",
        ),
    );
    providers.insert(
        "deepseek".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "https://api.deepseek.com",
            "DEEPSEEK_API_KEY",
            "deepseek-chat",
        ),
    );
    providers.insert(
        "kimi".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "https://api.moonshot.ai/v1",
            "MOONSHOT_API_KEY",
            "moonshot-v1-8k",
        ),
    );
    providers.insert(
        "kimi_cn".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "https://api.moonshot.cn/v1",
            "MOONSHOT_API_KEY",
            "moonshot-v1-8k",
        ),
    );
    providers.insert(
        "qwen".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "https://dashscope-us.aliyuncs.com/compatible-mode/v1",
            "DASHSCOPE_API_KEY",
            "qwen-plus",
        ),
    );
    providers.insert(
        "qwen_native".to_string(),
        provider(
            ProviderKind::DashscopeNative,
            "https://dashscope-us.aliyuncs.com/api/v1",
            "DASHSCOPE_API_KEY",
            "qwen-plus",
        ),
    );
    providers.insert(
        "qwen_cn".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "https://dashscope.aliyuncs.com/compatible-mode/v1",
            "DASHSCOPE_API_KEY",
            "qwen-plus",
        ),
    );
    providers.insert(
        "qwen_native_cn".to_string(),
        provider(
            ProviderKind::DashscopeNative,
            "https://dashscope.aliyuncs.com/api/v1",
            "DASHSCOPE_API_KEY",
            "qwen-plus",
        ),
    );
    providers.insert(
        "qwen_sg".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "https://dashscope-intl.aliyuncs.com/compatible-mode/v1",
            "DASHSCOPE_API_KEY",
            "qwen-plus",
        ),
    );
    providers.insert(
        "qwen_native_sg".to_string(),
        provider(
            ProviderKind::DashscopeNative,
            "https://dashscope-intl.aliyuncs.com/api/v1",
            "DASHSCOPE_API_KEY",
            "qwen-plus",
        ),
    );
    providers.insert(
        "glm".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "https://open.bigmodel.cn/api/paas/v4",
            "ZHIPUAI_API_KEY",
            "glm-4",
        ),
    );
    providers.insert(
        "glm_native".to_string(),
        provider(
            ProviderKind::ZhipuNative,
            "https://open.bigmodel.cn/api/paas/v4",
            "ZHIPUAI_API_KEY",
            "glm-4",
        ),
    );
    providers.insert(
        "minimax".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "https://api.minimax.chat/v1",
            "MINIMAX_API_KEY",
            "MiniMax-M2.5",
        ),
    );
    providers.insert(
        "minimax_native".to_string(),
        provider(
            ProviderKind::MiniMaxNative,
            "https://api.minimax.chat/v1",
            "MINIMAX_API_KEY",
            "MiniMax-M2.5",
        ),
    );
    providers.insert(
        "nvidia".to_string(),
        provider(
            ProviderKind::OpenAiCompatible,
            "https://integrate.api.nvidia.com/v1",
            "NVIDIA_API_KEY",
            "meta/llama-3.2-3b-instruct",
        ),
    );
    providers.insert(
        "minimax_anthropic".to_string(),
        provider(
            ProviderKind::Anthropic,
            "https://api.minimax.io/anthropic",
            "MINIMAX_API_KEY",
            "MiniMax-M2.5",
        ),
    );
    providers.insert(
        "anthropic".to_string(),
        provider(
            ProviderKind::Anthropic,
            "https://api.anthropic.com",
            "ANTHROPIC_API_KEY",
            "claude-3-5-sonnet-latest",
        ),
    );
    providers.insert(
        "gemini".to_string(),
        provider(
            ProviderKind::Gemini,
            "https://generativelanguage.googleapis.com/v1beta",
            "GEMINI_API_KEY",
            "gemini-1.5-flash",
        ),
    );
    providers
}

pub(super) fn default_llm_config() -> LlmConfig {
    LlmConfig {
        base_url: "http://127.0.0.1:11434/v1".to_string(),
        api_key_env: "OPENAI_API_KEY".to_string(),
        model: "gpt-4.1-mini".to_string(),
    }
}

pub(super) fn default_provider_config() -> ProviderConfig {
    provider(ProviderKind::OpenAiCompatible, "", "", "")
}

pub(super) fn default_router_config(
    default_provider: &str,
    _providers: &BTreeMap<String, ProviderConfig>,
) -> RouterConfig {
    RouterConfig {
        planning: RouteConfig {
            provider: default_provider.to_string(),
            model: "default".to_string(),
        },
        coding: RouteConfig {
            provider: default_provider.to_string(),
            model: "default".to_string(),
        },
        summary: RouteConfig {
            provider: default_provider.to_string(),
            model: "default".to_string(),
        },
    }
}

pub(super) fn default_route_config() -> RouteConfig {
    RouteConfig {
        provider: "ollama".to_string(),
        model: "default".to_string(),
    }
}

pub(super) fn default_skills_config() -> SkillsConfig {
    SkillsConfig {
        allowlist: Vec::new(),
        require_approval: false,
        auto_approve_readonly: true,
        experimental: false,
    }
}
