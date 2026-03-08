mod detect;
mod redact;

#[cfg(test)]
mod tests;

use rexos_kernel::security::{LeakMode, SecurityConfig};

use detect::{collect_matches, collect_sensitive_env_values, detector_labels, SensitiveEnvValue};
use redact::{mode_label, redact_content};

const BLOCKED_ERROR: &str = "tool output blocked by leak guard";

#[derive(Debug, Clone)]
pub(crate) struct LeakGuard {
    mode: LeakMode,
    sensitive_env_values: Vec<SensitiveEnvValue>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub(crate) struct LeakGuardAudit {
    pub(crate) mode: String,
    #[serde(default)]
    pub(crate) detectors: Vec<String>,
    #[serde(default)]
    pub(crate) redacted: bool,
    #[serde(default)]
    pub(crate) blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LeakGuardVerdict {
    Allowed {
        content: String,
        audit: Option<LeakGuardAudit>,
    },
    Blocked {
        error: String,
        audit: LeakGuardAudit,
    },
}

impl LeakGuard {
    pub(crate) fn from_security(security: &SecurityConfig) -> Self {
        Self {
            mode: security.leaks.mode.clone(),
            sensitive_env_values: collect_sensitive_env_values(),
        }
    }

    pub(crate) fn inspect_tool_output(&self, content: String) -> LeakGuardVerdict {
        if matches!(self.mode, LeakMode::Off) {
            return LeakGuardVerdict::Allowed {
                content,
                audit: None,
            };
        }

        let matches = collect_matches(&content, &self.sensitive_env_values);
        if matches.is_empty() {
            return LeakGuardVerdict::Allowed {
                content,
                audit: None,
            };
        }

        let detectors = detector_labels(&matches);
        let mode = mode_label(&self.mode).to_string();

        match self.mode {
            LeakMode::Off => LeakGuardVerdict::Allowed {
                content,
                audit: None,
            },
            LeakMode::Warn => LeakGuardVerdict::Allowed {
                content,
                audit: Some(LeakGuardAudit {
                    mode,
                    detectors,
                    redacted: false,
                    blocked: false,
                }),
            },
            LeakMode::Redact => LeakGuardVerdict::Allowed {
                content: redact_content(&content, &matches),
                audit: Some(LeakGuardAudit {
                    mode,
                    detectors,
                    redacted: true,
                    blocked: false,
                }),
            },
            LeakMode::Enforce => LeakGuardVerdict::Blocked {
                error: BLOCKED_ERROR.to_string(),
                audit: LeakGuardAudit {
                    mode,
                    detectors,
                    redacted: false,
                    blocked: true,
                },
            },
        }
    }
}
