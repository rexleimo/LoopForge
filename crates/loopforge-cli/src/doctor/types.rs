use std::time::Duration;

use rexos::paths::RexosPaths;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct DoctorOptions {
    pub paths: RexosPaths,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Ok,
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorCheck {
    pub id: String,
    pub status: CheckStatus,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorReport {
    pub checks: Vec<DoctorCheck>,
    pub summary: DoctorSummary,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorSummary {
    pub ok: u32,
    pub warn: u32,
    pub error: u32,
}
