mod env;
mod search;

use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SensitiveEnvValue {
    pub(super) detector: String,
    pub(super) value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LeakMatch {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) detector: String,
}

pub(super) fn collect_sensitive_env_values() -> Vec<SensitiveEnvValue> {
    env::collect_sensitive_env_values()
}

pub(super) fn collect_matches(
    content: &str,
    sensitive_env_values: &[SensitiveEnvValue],
) -> Vec<LeakMatch> {
    search::collect_matches(content, sensitive_env_values)
}

pub(super) fn detector_labels(matches: &[LeakMatch]) -> Vec<String> {
    let mut out = BTreeSet::new();
    for item in matches {
        out.insert(item.detector.clone());
    }
    out.into_iter().collect()
}
