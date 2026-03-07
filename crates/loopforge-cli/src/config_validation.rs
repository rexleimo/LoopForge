use rexos::{config::RexosConfig, paths::RexosPaths};

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ConfigValidationReport {
    pub(crate) valid: bool,
    pub(crate) config_path: String,
    pub(crate) errors: Vec<String>,
}

pub(crate) fn validate_config(paths: &RexosPaths) -> ConfigValidationReport {
    let config_path = paths.config_path();
    let display_path = config_path.display().to_string();
    let raw = match std::fs::read_to_string(&config_path) {
        Ok(raw) => raw,
        Err(e) => {
            return ConfigValidationReport {
                valid: false,
                config_path: display_path,
                errors: vec![format!("read config failed: {e}")],
            };
        }
    };

    let cfg: RexosConfig = match toml::from_str(&raw) {
        Ok(cfg) => cfg,
        Err(e) => {
            return ConfigValidationReport {
                valid: false,
                config_path: display_path,
                errors: vec![format!("parse config TOML failed: {e}")],
            };
        }
    };

    let mut errors = Vec::new();
    for (route_name, provider_name) in [
        ("planning", cfg.router.planning.provider.trim()),
        ("coding", cfg.router.coding.provider.trim()),
        ("summary", cfg.router.summary.provider.trim()),
    ] {
        if provider_name.is_empty() {
            errors.push(format!("router.{route_name}.provider is empty"));
            continue;
        }
        if !cfg.providers.contains_key(provider_name) {
            errors.push(format!(
                "router.{route_name}.provider references unknown provider '{provider_name}'"
            ));
        }
    }

    ConfigValidationReport {
        valid: errors.is_empty(),
        config_path: display_path,
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn validate_config_reports_success_for_default_config() {
        let tmp = tempdir().unwrap();
        let paths = RexosPaths {
            base_dir: tmp.path().join(".loopforge"),
        };
        paths.ensure_dirs().unwrap();
        RexosConfig::ensure_default(&paths).unwrap();

        let report = validate_config(&paths);
        assert!(report.valid, "expected config valid, got {report:?}");
        assert!(
            report.errors.is_empty(),
            "expected no errors, got {report:?}"
        );
    }

    #[test]
    fn validate_config_reports_parse_error_for_invalid_toml() {
        let tmp = tempdir().unwrap();
        let paths = RexosPaths {
            base_dir: tmp.path().join(".loopforge"),
        };
        paths.ensure_dirs().unwrap();
        std::fs::write(
            paths.config_path(),
            "[providers
broken = true",
        )
        .unwrap();

        let report = validate_config(&paths);
        assert!(!report.valid, "expected config invalid, got {report:?}");
        assert!(
            report
                .errors
                .iter()
                .any(|e| e.contains("parse config TOML")),
            "expected parse error, got {report:?}"
        );
    }
}
