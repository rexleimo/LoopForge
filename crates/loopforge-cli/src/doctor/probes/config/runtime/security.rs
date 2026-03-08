use rexos::config::RexosConfig;

use super::super::super::super::{CheckStatus, DoctorCheck};

pub(super) fn push_security_status(checks: &mut Vec<DoctorCheck>, cfg: &RexosConfig) {
    checks.push(DoctorCheck {
        id: "security.secrets.mode".to_string(),
        status: CheckStatus::Ok,
        message: match cfg.security.secrets.mode {
            rexos::security::SecretMode::EnvFirst => {
                "env_first (provider credentials resolve from host environment)".to_string()
            }
        },
    });

    checks.push(DoctorCheck {
        id: "security.leaks.mode".to_string(),
        status: match cfg.security.leaks.mode {
            rexos::security::LeakMode::Off | rexos::security::LeakMode::Warn => CheckStatus::Warn,
            rexos::security::LeakMode::Redact | rexos::security::LeakMode::Enforce => {
                CheckStatus::Ok
            }
        },
        message: match cfg.security.leaks.mode {
            rexos::security::LeakMode::Off => {
                "off (tool output is not scanned for secrets)".to_string()
            }
            rexos::security::LeakMode::Warn => {
                "warn (detects likely secrets but still forwards raw output)".to_string()
            }
            rexos::security::LeakMode::Redact => {
                "redact (masks detected secrets before persistence and follow-up model calls)"
                    .to_string()
            }
            rexos::security::LeakMode::Enforce => {
                "enforce (blocks tool output when likely secrets are detected)".to_string()
            }
        },
    });

    let egress_rules = cfg.security.egress.rules.len();
    checks.push(DoctorCheck {
        id: "security.egress.rules".to_string(),
        status: if egress_rules == 0 {
            CheckStatus::Warn
        } else {
            CheckStatus::Ok
        },
        message: if egress_rules == 0 {
            "no allowlist rules configured; network tools still rely on baseline SSRF/private-network guards only".to_string()
        } else {
            format!("{egress_rules} outbound allowlist rule(s) configured")
        },
    });
}
