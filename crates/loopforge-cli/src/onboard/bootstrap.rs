use anyhow::Context;
use std::path::PathBuf;

use rexos::{config::RexosConfig, memory::MemoryStore, paths::RexosPaths};

use super::{flow_types::OnboardBootstrap, resolve_onboard_prompt, OnboardStarter};

pub(super) fn bootstrap_onboard(
    workspace: PathBuf,
    prompt: Option<&str>,
    starter: OnboardStarter,
) -> anyhow::Result<OnboardBootstrap> {
    let paths = RexosPaths::discover()?;
    paths.ensure_dirs()?;
    RexosConfig::ensure_default(&paths)?;
    MemoryStore::open_or_create(&paths)?;
    println!("Initialized {}", paths.base_dir.display());

    std::fs::create_dir_all(&workspace)
        .with_context(|| format!("create workspace: {}", workspace.display()))?;
    println!("workspace ready: {}", workspace.display());

    Ok(OnboardBootstrap {
        paths,
        workspace,
        effective_prompt: resolve_onboard_prompt(prompt, starter),
        starter,
    })
}
