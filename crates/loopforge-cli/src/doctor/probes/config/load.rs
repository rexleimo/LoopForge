use std::path::Path;

use rexos::config::RexosConfig;

use super::super::super::{CheckStatus, DoctorCheck, DoctorOptions};

pub(super) fn load_config_checks(
    checks: &mut Vec<DoctorCheck>,
    opts: &DoctorOptions,
) -> Option<RexosConfig> {
    checks.push(DoctorCheck {
        id: "paths.base_dir".to_string(),
        status: CheckStatus::Ok,
        message: opts.paths.base_dir.display().to_string(),
    });

    let config_path = opts.paths.config_path();
    let db_path = opts.paths.db_path();

    checks.push(path_check(
        "paths.config",
        &config_path,
        " (missing; run `loopforge init`)",
    ));
    checks.push(path_check(
        "paths.db",
        &db_path,
        " (missing; run `loopforge init`)",
    ));

    if !config_path.exists() {
        return None;
    }

    match RexosConfig::load(&opts.paths) {
        Ok(cfg) => {
            checks.push(DoctorCheck {
                id: "config.parse".to_string(),
                status: CheckStatus::Ok,
                message: "config.toml parsed".to_string(),
            });
            Some(cfg)
        }
        Err(err) => {
            checks.push(DoctorCheck {
                id: "config.parse".to_string(),
                status: CheckStatus::Error,
                message: err.to_string(),
            });
            None
        }
    }
}

pub(super) fn path_check(id: &str, path: &Path, missing_suffix: &str) -> DoctorCheck {
    DoctorCheck {
        id: id.to_string(),
        status: if path.exists() {
            CheckStatus::Ok
        } else {
            CheckStatus::Warn
        },
        message: format!(
            "{}{}",
            path.display(),
            if path.exists() { "" } else { missing_suffix }
        ),
    }
}
