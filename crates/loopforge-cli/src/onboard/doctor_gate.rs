use crate::doctor;

pub(super) fn is_onboard_blocking_doctor_error(check: &doctor::DoctorCheck) -> bool {
    if check.status != doctor::CheckStatus::Error {
        return false;
    }
    check.id == "config.parse" || check.id.starts_with("router.")
}
