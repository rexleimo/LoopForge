mod next_actions;
mod summary;

use super::{DoctorCheck, DoctorSummary};

pub(super) fn summarize(checks: &[DoctorCheck]) -> DoctorSummary {
    summary::summarize(checks)
}

pub(super) fn derive_next_actions(checks: &[DoctorCheck]) -> Vec<String> {
    next_actions::derive_next_actions(checks)
}
