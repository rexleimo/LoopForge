use std::path::PathBuf;

use rexos::paths::RexosPaths;

use super::{outcome::OnboardReportBase, OnboardStarter};

pub(super) struct OnboardBootstrap {
    pub(super) paths: RexosPaths,
    pub(super) workspace: PathBuf,
    pub(super) effective_prompt: String,
    pub(super) starter: OnboardStarter,
}

pub(super) struct PreparedOnboard {
    pub(super) paths: RexosPaths,
    pub(super) workspace: PathBuf,
    pub(super) effective_prompt: String,
    pub(super) starter: OnboardStarter,
    pub(super) report_base: OnboardReportBase,
}
