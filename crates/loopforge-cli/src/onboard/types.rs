use std::collections::BTreeMap;

use crate::doctor;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub(crate) struct OnboardMetrics {
    pub(crate) attempted_first_task: u64,
    pub(crate) first_task_success: u64,
    pub(crate) first_task_failed: u64,
    pub(crate) failure_by_category: BTreeMap<String, u64>,
    pub(crate) updated_at_ms: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct OnboardEvent {
    pub(crate) ts_ms: i64,
    pub(crate) workspace: String,
    pub(crate) session_id: String,
    pub(crate) outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) failure_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum OnboardStarter {
    Hello,
    WorkspaceBrief,
    RepoOnboarding,
}

impl OnboardStarter {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            OnboardStarter::Hello => "hello",
            OnboardStarter::WorkspaceBrief => "workspace-brief",
            OnboardStarter::RepoOnboarding => "repo-onboarding",
        }
    }

    pub(crate) fn default_prompt(self) -> &'static str {
        match self {
            OnboardStarter::Hello => "Create hello.txt with the word hi",
            OnboardStarter::WorkspaceBrief => {
                "Create notes/workspace-brief.md with: what this workspace is for, 3 risks, and 3 next actions."
            }
            OnboardStarter::RepoOnboarding => {
                "Read README.md plus the most important project metadata files you can find. Create notes/repo-onboarding.md with: project purpose, how to run it, what to verify first, and 3 next actions."
            }
        }
    }

    pub(crate) fn description(self) -> &'static str {
        match self {
            OnboardStarter::Hello => "Minimal smoke check that proves file creation works.",
            OnboardStarter::WorkspaceBrief => {
                "Create a short workspace brief with risks and next actions."
            }
            OnboardStarter::RepoOnboarding => {
                "Read the repo and produce an onboarding brief for real project work."
            }
        }
    }

    pub(crate) fn expected_artifact(self) -> &'static str {
        match self {
            OnboardStarter::Hello => "hello.txt",
            OnboardStarter::WorkspaceBrief => "notes/workspace-brief.md",
            OnboardStarter::RepoOnboarding => "notes/repo-onboarding.md",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct OnboardStarterSuggestion {
    pub(crate) starter: String,
    pub(crate) description: String,
    pub(crate) command: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct OnboardTaskReport {
    pub(crate) status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) failure_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct OnboardReportArtifact {
    pub(crate) generated_at_ms: i64,
    pub(crate) workspace: String,
    pub(crate) config_path: String,
    pub(crate) config_valid: bool,
    pub(crate) starter: String,
    pub(crate) effective_prompt: String,
    pub(crate) doctor_summary: doctor::DoctorSummary,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) doctor_next_actions: Vec<String>,
    pub(crate) task: OnboardTaskReport,
    pub(crate) recommended_next_command: String,
    pub(crate) starter_suggestions: Vec<OnboardStarterSuggestion>,
}
