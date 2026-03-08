use std::process::Command;

use super::super::{CheckStatus, DoctorCheck};

pub(super) fn push_command_checks(checks: &mut Vec<DoctorCheck>) {
    checks.push(check_command("git", &["--version"], "tools.git", true));
    checks.push(check_command(
        "docker",
        &["--version"],
        "tools.docker",
        false,
    ));
}

fn check_command(command: &str, args: &[&str], id: &str, required: bool) -> DoctorCheck {
    match Command::new(command).args(args).output() {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            DoctorCheck {
                id: id.to_string(),
                status: CheckStatus::Ok,
                message: if stdout.is_empty() {
                    format!("{command} available")
                } else {
                    stdout
                },
            }
        }
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let mut msg = format!("{command} returned non-zero");
            if !stdout.is_empty() {
                msg.push_str(&format!("; stdout={stdout}"));
            }
            if !stderr.is_empty() {
                msg.push_str(&format!("; stderr={stderr}"));
            }
            DoctorCheck {
                id: id.to_string(),
                status: if required {
                    CheckStatus::Error
                } else {
                    CheckStatus::Warn
                },
                message: msg,
            }
        }
        Err(err) => DoctorCheck {
            id: id.to_string(),
            status: if required {
                CheckStatus::Error
            } else {
                CheckStatus::Warn
            },
            message: format!("{command} not found ({err})"),
        },
    }
}
