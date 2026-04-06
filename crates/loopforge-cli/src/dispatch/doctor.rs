use rexos::paths::RexosPaths;

use crate::doctor;

pub(super) async fn run(json: bool, strict: bool, timeout_ms: u64) -> anyhow::Result<()> {
    let paths = RexosPaths::discover()?;
    let report = doctor::run_doctor(doctor::DoctorOptions {
        paths,
        timeout: std::time::Duration::from_millis(timeout_ms),
    })
    .await?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&build_doctor_json(&report)?)?
        );
    } else {
        println!("{}", report.to_text());
    }

    let code = report.exit_code(strict);
    if code != 0 {
        std::process::exit(code);
    }
    Ok(())
}

fn build_doctor_json(report: &doctor::DoctorReport) -> anyhow::Result<serde_json::Value> {
    Ok(serde_json::to_value(report)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doctor::{CheckStatus, DoctorCheck, DoctorReport, DoctorSummary};
    use serde_json::json;

    #[test]
    fn build_doctor_json_keeps_expected_shape_and_omits_empty_fields() {
        let report = DoctorReport {
            checks: vec![
                DoctorCheck {
                    id: "alpha".to_string(),
                    status: CheckStatus::Ok,
                    message: String::new(),
                },
                DoctorCheck {
                    id: "beta".to_string(),
                    status: CheckStatus::Warn,
                    message: "missing env".to_string(),
                },
            ],
            summary: DoctorSummary {
                ok: 1,
                warn: 1,
                error: 0,
            },
            next_actions: Vec::new(),
        };

        let out = build_doctor_json(&report).unwrap();
        assert_eq!(
            out,
            json!({
                "checks": [
                    { "id": "alpha", "status": "ok" },
                    { "id": "beta", "status": "warn", "message": "missing env" },
                ],
                "summary": { "ok": 1, "warn": 1, "error": 0 },
            })
        );
    }
}
