use super::doctor_gate::is_onboard_blocking_doctor_error;
use super::flow_types::{OnboardBootstrap, PreparedOnboard};
use super::outcome::{emit_onboard_report, OnboardReportBase};
use super::OnboardTaskReport;
use crate::{config_validation::validate_config, doctor};

pub(super) async fn run_preflight(
    bootstrap: OnboardBootstrap,
    timeout_ms: u64,
) -> anyhow::Result<PreparedOnboard> {
    let config_report = validate_config(&bootstrap.paths);
    if !config_report.valid {
        println!("config invalid: {}", config_report.config_path);
        for err in &config_report.errors {
            println!("- {err}");
        }

        let report_base = OnboardReportBase::new(
            &bootstrap.workspace,
            config_report.config_path.clone(),
            false,
            bootstrap.starter,
            &bootstrap.effective_prompt,
            doctor::DoctorSummary {
                ok: 0,
                warn: 0,
                error: 1,
            },
            vec![format!(
                "Fix `{}` and rerun `loopforge config validate`.",
                config_report.config_path
            )],
        );
        let report = report_base.build_report(
            OnboardTaskReport {
                status: "blocked".to_string(),
                session_id: None,
                failure_category: Some("config_invalid".to_string()),
                error: Some(config_report.errors.join("; ")),
            },
            "loopforge config validate".to_string(),
        );
        emit_onboard_report(&bootstrap.workspace, &report)?;
        std::process::exit(1);
    }
    println!("config valid: {}", config_report.config_path);

    let doctor_report = doctor::run_doctor(doctor::DoctorOptions {
        paths: bootstrap.paths.clone(),
        timeout: std::time::Duration::from_millis(timeout_ms),
    })
    .await?;
    println!("{}", doctor_report.to_text());

    let report_base = OnboardReportBase::new(
        &bootstrap.workspace,
        config_report.config_path.clone(),
        true,
        bootstrap.starter,
        &bootstrap.effective_prompt,
        doctor_report.summary.clone(),
        doctor_report.next_actions.clone(),
    );

    let blocking_errors: Vec<&doctor::DoctorCheck> = doctor_report
        .checks
        .iter()
        .filter(|check| is_onboard_blocking_doctor_error(check))
        .collect();
    if !blocking_errors.is_empty() {
        eprintln!("onboard blocked by critical setup errors:");
        for check in &blocking_errors {
            eprintln!("- {}: {}", check.id, check.message);
        }

        let report = report_base.build_report(
            OnboardTaskReport {
                status: "blocked".to_string(),
                session_id: None,
                failure_category: Some("doctor_blocked".to_string()),
                error: Some(
                    blocking_errors
                        .iter()
                        .map(|check| format!("{}: {}", check.id, check.message))
                        .collect::<Vec<_>>()
                        .join("; "),
                ),
            },
            "loopforge doctor".to_string(),
        );
        emit_onboard_report(&bootstrap.workspace, &report)?;
        std::process::exit(1);
    }

    let non_blocking_errors = doctor_report
        .checks
        .iter()
        .filter(|check| check.status == doctor::CheckStatus::Error)
        .count()
        .saturating_sub(blocking_errors.len());
    if non_blocking_errors > 0 {
        eprintln!(
            "onboard: continuing despite {} non-blocking doctor error(s)",
            non_blocking_errors
        );
    }

    Ok(PreparedOnboard {
        paths: bootstrap.paths,
        workspace: bootstrap.workspace,
        effective_prompt: bootstrap.effective_prompt,
        starter: bootstrap.starter,
        report_base,
    })
}
