use rexos::config::{ProviderKind, RexosConfig};

use super::model_probe::{fetch_openai_compat_models, select_onboard_model};

pub(super) async fn maybe_select_ollama_model(cfg: &mut RexosConfig, timeout_ms: u64) {
    if cfg.router.coding.provider.trim() != "ollama" {
        return;
    }

    let maybe_ollama = cfg.providers.get("ollama").cloned();
    if let Some(ollama) = maybe_ollama {
        if ollama.kind == ProviderKind::OpenAiCompatible {
            if let Ok(models) = fetch_openai_compat_models(&ollama.base_url, timeout_ms).await {
                if let Some(selected) = select_onboard_model(&ollama.default_model, &models) {
                    if selected != ollama.default_model {
                        if let Some(provider) = cfg.providers.get_mut("ollama") {
                            provider.default_model = selected.clone();
                        }
                        println!(
                            "onboard: ollama default model '{}' not available, using '{}'",
                            ollama.default_model, selected
                        );
                    }
                }
            }
        }
    }
}
