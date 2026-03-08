use std::collections::BTreeSet;

use super::SensitiveEnvValue;

pub(super) fn collect_sensitive_env_values() -> Vec<SensitiveEnvValue> {
    let mut out = Vec::new();
    let mut seen_values = BTreeSet::new();

    for (name, value) in std::env::vars() {
        if !is_sensitive_env_name(&name) {
            continue;
        }
        if value.trim().len() < 8 || value.contains(char::is_whitespace) {
            continue;
        }
        if !seen_values.insert(value.clone()) {
            continue;
        }
        out.push(SensitiveEnvValue {
            detector: format!("env:{name}"),
            value,
        });
    }

    out
}

fn is_sensitive_env_name(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    if matches!(upper.as_str(), "PATH" | "PWD" | "HOME" | "SHELL" | "TERM") {
        return false;
    }

    upper.contains("TOKEN")
        || upper.contains("SECRET")
        || upper.contains("PASSWORD")
        || upper.contains("BEARER")
        || upper.ends_with("_KEY")
        || upper.contains("_KEY_")
        || upper.contains("API_KEY")
}
