use std::path::Path;

use rexos::config::RexosConfig;

use super::load::path_check;
use super::runtime::provider_env_status;
use crate::doctor::{CheckStatus, DoctorCheck};

#[test]
fn path_check_marks_missing_paths_as_warn() {
    let check = path_check(
        "paths.config",
        Path::new("/definitely/missing/config.toml"),
        " missing",
    );
    assert_eq!(check.id, "paths.config");
    assert_eq!(check.status, CheckStatus::Warn);
    assert!(check.message.ends_with(" missing"), "{}", check.message);
}

#[test]
fn provider_env_status_warns_when_required_env_missing() {
    let mut checks = Vec::<DoctorCheck>::new();
    let cfg = RexosConfig::default();
    provider_env_status(&mut checks, &cfg);
    let check = checks
        .iter()
        .find(|check| check.id == "providers.api_keys")
        .unwrap();
    assert_eq!(check.status, CheckStatus::Warn);
    assert!(
        check.message.contains("missing env vars:"),
        "{}",
        check.message
    );
}
