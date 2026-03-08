use rexos::{config::RexosConfig, memory::MemoryStore, paths::RexosPaths};

pub(crate) fn ensure_paths() -> anyhow::Result<RexosPaths> {
    let paths = RexosPaths::discover()?;
    paths.ensure_dirs()?;
    RexosConfig::ensure_default(&paths)?;
    Ok(paths)
}

pub(crate) fn load_runtime_config() -> anyhow::Result<(RexosPaths, RexosConfig)> {
    let paths = ensure_paths()?;
    let cfg = RexosConfig::load(&paths)?;
    Ok((paths, cfg))
}

pub(crate) fn open_memory(paths: &RexosPaths) -> anyhow::Result<MemoryStore> {
    MemoryStore::open_or_create(paths)
}

pub(crate) fn build_agent_runtime(
    paths: &RexosPaths,
    cfg: RexosConfig,
) -> anyhow::Result<rexos::agent::AgentRuntime> {
    let memory = open_memory(paths)?;
    let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg)?;
    let security = cfg.security.clone();
    let router = rexos::router::ModelRouter::new(cfg.router);
    Ok(rexos::agent::AgentRuntime::new_with_security_config(
        memory, llms, router, security,
    ))
}

pub(crate) fn load_agent_runtime() -> anyhow::Result<(RexosPaths, rexos::agent::AgentRuntime)> {
    let (paths, cfg) = load_runtime_config()?;
    let agent = build_agent_runtime(&paths, cfg)?;
    Ok((paths, agent))
}

pub(crate) fn load_dispatcher() -> anyhow::Result<rexos::agent::OutboxDispatcher> {
    let paths = ensure_paths()?;
    let memory = open_memory(&paths)?;
    rexos::agent::OutboxDispatcher::new(memory)
}
