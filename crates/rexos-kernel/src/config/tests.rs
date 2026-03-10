use super::defaults::{default_providers, default_router_config};
use super::*;

#[test]
fn default_config_serializes() {
    let cfg = RexosConfig::default();
    let toml_str = toml::to_string_pretty(&cfg).unwrap();
    assert!(toml_str.contains("[providers.ollama]"));
    assert!(toml_str.contains("[providers.deepseek]"));
    assert!(toml_str.contains("[providers.kimi]"));
    assert!(toml_str.contains("[providers.qwen]"));
    assert!(toml_str.contains("[providers.qwen_native]"));
    assert!(toml_str.contains("[providers.glm]"));
    assert!(toml_str.contains("[providers.glm_native]"));
    assert!(toml_str.contains("[providers.minimax]"));
    assert!(toml_str.contains("[providers.minimax_native]"));
    assert!(toml_str.contains("[providers.nvidia]"));
    assert!(toml_str.contains("[providers.anthropic]"));
    assert!(toml_str.contains("[providers.gemini]"));
    assert!(toml_str.contains("[providers.bedrock]"));
    assert!(toml_str.contains("kind = \"openai_compatible\""));
    assert!(toml_str.contains("kind = \"dashscope_native\""));
    assert!(toml_str.contains("kind = \"zhipu_native\""));
    assert!(toml_str.contains("kind = \"minimax_native\""));
    assert!(toml_str.contains("kind = \"bedrock\""));
    assert!(toml_str.contains("base_url"));
    assert!(toml_str.contains("api_key_env"));
    assert!(toml_str.contains("default_model"));

    assert!(toml_str.contains("[router.planning]"));
    assert!(toml_str.contains("provider = \"ollama\""));
    assert!(toml_str.contains("[router.coding]"));
    assert!(toml_str.contains("[router.summary]"));
    assert!(toml_str.contains("[security.secrets]"));
    assert!(toml_str.contains("mode = \"env_first\""));
    assert!(toml_str.contains("[security.leaks]"));
    assert!(toml_str.contains("[security.egress]"));
}

#[test]
fn default_providers_include_minimax_anthropic_preset() {
    let providers = default_providers();
    let preset = providers.get("minimax_anthropic").unwrap();
    assert_eq!(preset.kind, ProviderKind::Anthropic);
    assert_eq!(preset.base_url, "https://api.minimax.io/anthropic");
    assert_eq!(preset.api_key_env, "MINIMAX_API_KEY");
    assert_eq!(preset.default_model, "MiniMax-M2.5");
}

#[test]
fn default_router_config_uses_default_model_for_all_routes() {
    let providers = default_providers();
    let router = default_router_config("ollama", &providers);

    assert_eq!(router.planning.provider, "ollama");
    assert_eq!(router.planning.model, "default");
    assert_eq!(router.coding.provider, "ollama");
    assert_eq!(router.coding.model, "default");
    assert_eq!(router.summary.provider, "ollama");
    assert_eq!(router.summary.model, "default");
}

#[test]
fn bedrock_provider_parses_optional_aws_config_table() {
    let parsed: RexosConfig = toml::from_str(
        r#"
[providers.bedrock]
kind = "bedrock"
default_model = "anthropic.claude-3-5-sonnet-20241022-v2:0"

[providers.bedrock.aws_bedrock]
region = "us-east-1"
"#,
    )
    .unwrap();

    let bedrock = parsed.providers.get("bedrock").unwrap();
    assert_eq!(bedrock.kind, ProviderKind::Bedrock);
    assert_eq!(
        bedrock.default_model,
        "anthropic.claude-3-5-sonnet-20241022-v2:0"
    );
    assert_eq!(
        bedrock
            .aws_bedrock
            .as_ref()
            .map(|cfg| cfg.region.as_str()),
        Some("us-east-1")
    );
}

#[test]
fn minimax_presets_use_official_base_url() {
    let cfg = RexosConfig::default();

    let minimax = cfg.providers.get("minimax").unwrap();
    assert_eq!(minimax.base_url, "https://api.minimax.chat/v1");

    let minimax_native = cfg.providers.get("minimax_native").unwrap();
    assert_eq!(minimax_native.base_url, "https://api.minimax.chat/v1");
}

#[test]
fn nvidia_preset_uses_nim_base_url() {
    let cfg = RexosConfig::default();

    let nvidia = cfg.providers.get("nvidia").unwrap();
    assert_eq!(nvidia.base_url, "https://integrate.api.nvidia.com/v1");
}

#[test]
fn security_config_parses_table_values() {
    use crate::security::{LeakMode, SecretMode};

    let parsed: RexosConfig = toml::from_str(
        r#"
[security.secrets]
mode = "env_first"

[security.leaks]
mode = "warn"

[[security.egress.rules]]
tool = "web_fetch"
host = "docs.rs"
path_prefix = "/"
methods = ["GET"]
"#,
    )
    .unwrap();

    assert_eq!(parsed.security.secrets.mode, SecretMode::EnvFirst);
    assert_eq!(parsed.security.leaks.mode, LeakMode::Warn);
    assert_eq!(parsed.security.egress.rules.len(), 1);
    assert_eq!(parsed.security.egress.rules[0].tool, "web_fetch");
    assert_eq!(parsed.security.egress.rules[0].host, "docs.rs");
    assert_eq!(parsed.security.egress.rules[0].path_prefix, "/");
    assert_eq!(parsed.security.egress.rules[0].methods, vec!["GET"]);
}

#[test]
fn skills_config_defaults_when_table_missing() {
    let parsed: SkillsConfigWrapper = toml::from_str("[llm]\nmodel = \"x\"").unwrap();
    assert!(parsed.skills.allowlist.is_empty());
    assert!(!parsed.skills.require_approval);
    assert!(parsed.skills.auto_approve_readonly);
}

#[test]
fn skills_config_parses_table_values() {
    let parsed: SkillsConfigWrapper = toml::from_str(
        r#"
[skills]
allowlist = ["safe-skill", "qa-helper"]
require_approval = true
auto_approve_readonly = false
experimental = true
"#,
    )
    .unwrap();
    assert_eq!(parsed.skills.allowlist, vec!["safe-skill", "qa-helper"]);
    assert!(parsed.skills.require_approval);
    assert!(!parsed.skills.auto_approve_readonly);
    assert!(parsed.skills.experimental);
}

#[test]
fn ensure_default_writes_skills_table() {
    let tmp = tempfile::tempdir().unwrap();
    let paths = crate::paths::RexosPaths {
        base_dir: tmp.path().join(".loopforge"),
    };
    paths.ensure_dirs().unwrap();

    RexosConfig::ensure_default(&paths).unwrap();
    let raw = std::fs::read_to_string(paths.config_path()).unwrap();
    assert!(raw.contains("[skills]"));
    assert!(raw.contains("require_approval = false"));
}
