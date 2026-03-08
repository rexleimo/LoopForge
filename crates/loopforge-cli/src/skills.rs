mod discovery;
mod doctor;
mod permissions;

#[cfg(test)]
mod tests;

use serde::Serialize;

pub(crate) use discovery::{find_skill, list_skills, read_skill_entry, source_name};
pub(crate) use doctor::doctor;
pub(crate) use permissions::permission_tools;

#[derive(Debug, Clone, Serialize)]
pub struct SkillListItem {
    pub name: String,
    pub version: String,
    pub source: String,
    pub root_dir: String,
    pub entry_path: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillsDoctorLevel {
    Warn,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkillsDoctorIssue {
    pub level: SkillsDoctorLevel,
    pub id: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkillsDoctorReport {
    pub ok: bool,
    pub discovered_count: usize,
    pub issues: Vec<SkillsDoctorIssue>,
}
