use rexos::paths::RexosPaths;

use super::json_output::{print_pretty_json, to_json_value};
use crate::{cli::ConfigCommand, config_validation::validate_config};

pub(super) fn run(command: ConfigCommand) -> anyhow::Result<()> {
    match command {
        ConfigCommand::Validate { json } => {
            let paths = RexosPaths::discover()?;
            let report = validate_config(&paths);
            if json {
                print_pretty_json(&build_config_validate_json(&report)?)?;
            } else if report.valid {
                println!("config valid: {}", report.config_path);
            } else {
                println!("config invalid: {}", report.config_path);
                for err in &report.errors {
                    println!("- {err}");
                }
            }

            if !report.valid {
                std::process::exit(1);
            }
            Ok(())
        }
    }
}

fn build_config_validate_json(
    report: &crate::config_validation::ConfigValidationReport,
) -> anyhow::Result<serde_json::Value> {
    to_json_value(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config_validation::ConfigValidationReport;
    use serde_json::json;

    #[test]
    fn build_config_validate_json_keeps_expected_shape() {
        let report = ConfigValidationReport {
            valid: false,
            config_path: "/tmp/config.toml".to_string(),
            errors: vec!["router.planning.provider is empty".to_string()],
        };

        let out = build_config_validate_json(&report).unwrap();
        assert_eq!(
            out,
            json!({
                "valid": false,
                "config_path": "/tmp/config.toml",
                "errors": ["router.planning.provider is empty"],
            })
        );
    }
}
