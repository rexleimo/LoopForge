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
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("{}", report.to_text());
    }

    let code = report.exit_code(strict);
    if code != 0 {
        std::process::exit(code);
    }
    Ok(())
}
