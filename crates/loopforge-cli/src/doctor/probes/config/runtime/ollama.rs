use rexos::config::{ProviderKind, RexosConfig};

use super::super::super::super::{CheckStatus, DoctorCheck};

pub(super) async fn push_local_ollama_check(
    checks: &mut Vec<DoctorCheck>,
    cfg: &RexosConfig,
    http: &reqwest::Client,
) {
    let Some(ollama) = cfg.providers.get("ollama") else {
        return;
    };

    if ollama.kind != ProviderKind::OpenAiCompatible || !ollama.api_key_env.trim().is_empty() {
        return;
    }

    let Ok(url) = reqwest::Url::parse(&ollama.base_url) else {
        return;
    };
    let is_loopback = matches!(
        url.host_str(),
        Some("127.0.0.1") | Some("localhost") | Some("::1")
    );
    if !is_loopback {
        return;
    }

    let probe = format!("{}/models", ollama.base_url.trim_end_matches('/'));
    let response = http.get(&probe).send().await;
    match response {
        Ok(response) if response.status().is_success() => checks.push(DoctorCheck {
            id: "ollama.http".to_string(),
            status: CheckStatus::Ok,
            message: format!("GET {probe} -> {}", response.status()),
        }),
        Ok(response) => checks.push(DoctorCheck {
            id: "ollama.http".to_string(),
            status: CheckStatus::Warn,
            message: format!("GET {probe} -> {}", response.status()),
        }),
        Err(err) => checks.push(DoctorCheck {
            id: "ollama.http".to_string(),
            status: CheckStatus::Warn,
            message: format!("GET {probe} failed: {err}"),
        }),
    }
}
