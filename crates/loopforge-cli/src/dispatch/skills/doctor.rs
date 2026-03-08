use std::path::PathBuf;

use crate::skills;

pub(super) fn run_doctor(workspace: PathBuf, json: bool, strict: bool) -> anyhow::Result<()> {
    let report = skills::doctor(&workspace)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("discovered_skills: {}", report.discovered_count);
        if report.issues.is_empty() {
            println!("doctor: ok");
        } else {
            for issue in &report.issues {
                let level = match issue.level {
                    skills::SkillsDoctorLevel::Warn => "warn",
                    skills::SkillsDoctorLevel::Error => "error",
                };
                if let Some(path) = &issue.path {
                    println!("[{level}] {}: {} ({path})", issue.id, issue.message);
                } else {
                    println!("[{level}] {}: {}", issue.id, issue.message);
                }
            }
        }
    }

    let has_error = report
        .issues
        .iter()
        .any(|issue| matches!(issue.level, skills::SkillsDoctorLevel::Error));
    let has_warn = report
        .issues
        .iter()
        .any(|issue| matches!(issue.level, skills::SkillsDoctorLevel::Warn));
    if has_error || (strict && has_warn) {
        std::process::exit(1);
    }
    Ok(())
}
