use rexos::paths::RexosPaths;

use crate::{cli::ConfigCommand, config_validation::validate_config};

pub(super) fn run(command: ConfigCommand) -> anyhow::Result<()> {
    match command {
        ConfigCommand::Validate { json } => {
            let paths = RexosPaths::discover()?;
            let report = validate_config(&paths);
            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
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
