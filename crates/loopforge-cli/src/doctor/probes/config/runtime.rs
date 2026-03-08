mod ollama;
mod providers;
mod router;
mod security;

use rexos::config::RexosConfig;

use super::super::super::DoctorCheck;

pub(super) async fn push_runtime_checks(
    checks: &mut Vec<DoctorCheck>,
    cfg: &RexosConfig,
    http: &reqwest::Client,
) {
    router::push_router_status(checks, cfg);
    provider_env_status(checks, cfg);
    security::push_security_status(checks, cfg);
    ollama::push_local_ollama_check(checks, cfg, http).await;
}

pub(super) fn provider_env_status(checks: &mut Vec<DoctorCheck>, cfg: &RexosConfig) {
    providers::provider_env_status(checks, cfg)
}
