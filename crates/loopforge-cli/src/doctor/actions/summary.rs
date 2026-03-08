use crate::doctor::{CheckStatus, DoctorCheck, DoctorSummary};

pub(super) fn summarize(checks: &[DoctorCheck]) -> DoctorSummary {
    let mut ok = 0u32;
    let mut warn = 0u32;
    let mut error = 0u32;
    for check in checks {
        match check.status {
            CheckStatus::Ok => ok += 1,
            CheckStatus::Warn => warn += 1,
            CheckStatus::Error => error += 1,
        }
    }
    DoctorSummary { ok, warn, error }
}
