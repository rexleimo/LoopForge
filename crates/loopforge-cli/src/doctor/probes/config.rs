mod load;
mod runtime;

#[cfg(test)]
mod tests;

use rexos::config::RexosConfig;

use super::super::DoctorCheck;
use super::DoctorOptions;

pub(super) fn load_config_checks(
    checks: &mut Vec<DoctorCheck>,
    opts: &DoctorOptions,
) -> Option<RexosConfig> {
    load::load_config_checks(checks, opts)
}

pub(super) async fn push_runtime_checks(
    checks: &mut Vec<DoctorCheck>,
    cfg: &RexosConfig,
    http: &reqwest::Client,
) {
    runtime::push_runtime_checks(checks, cfg, http).await;
}
