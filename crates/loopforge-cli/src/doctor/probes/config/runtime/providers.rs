use rexos::config::RexosConfig;

use super::super::super::super::{CheckStatus, DoctorCheck};

pub(super) fn provider_env_status(checks: &mut Vec<DoctorCheck>, cfg: &RexosConfig) {
    let mut missing = Vec::new();
    for (name, provider) in &cfg.providers {
        if !provider.api_key_env.trim().is_empty() && std::env::var(&provider.api_key_env).is_err()
        {
            missing.push(format!("{name} -> {}", provider.api_key_env));
        }
    }

    checks.push(DoctorCheck {
        id: "providers.api_keys".to_string(),
        status: if missing.is_empty() {
            CheckStatus::Ok
        } else {
            CheckStatus::Warn
        },
        message: if missing.is_empty() {
            "all required provider env vars are set".to_string()
        } else {
            format!(
                "missing env vars: {}",
                missing.into_iter().take(8).collect::<Vec<_>>().join(", ")
            )
        },
    });
}
