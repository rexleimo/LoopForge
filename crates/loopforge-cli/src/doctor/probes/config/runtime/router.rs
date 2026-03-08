use rexos::config::RexosConfig;

use super::super::super::super::{CheckStatus, DoctorCheck};

pub(super) fn push_router_status(checks: &mut Vec<DoctorCheck>, cfg: &RexosConfig) {
    for (kind, route) in [
        ("planning", &cfg.router.planning),
        ("coding", &cfg.router.coding),
        ("summary", &cfg.router.summary),
    ] {
        let id = format!("router.{kind}.provider");
        if cfg.providers.contains_key(&route.provider) {
            checks.push(DoctorCheck {
                id,
                status: CheckStatus::Ok,
                message: route.provider.clone(),
            });
        } else {
            checks.push(DoctorCheck {
                id,
                status: CheckStatus::Error,
                message: format!(
                    "unknown provider '{}' (defined: [{}])",
                    route.provider,
                    cfg.providers.keys().cloned().collect::<Vec<_>>().join(", ")
                ),
            });
        }
    }
}
