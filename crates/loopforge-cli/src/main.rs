use anyhow::Context;
use clap::Parser;
use std::path::PathBuf;

use rexos::{config::RexosConfig, memory::MemoryStore, paths::RexosPaths};

mod acp;
mod config_validation;
mod doctor;
mod onboard;
mod release_check;
mod skills;

use acp::{load_acp_checkpoints, load_acp_events};
use config_validation::validate_config;
use onboard::OnboardStarter;
use release_check::{format_release_check_report, run_release_check};
#[derive(Debug, Parser)]
#[command(name = "loopforge")]
#[command(
    about = "LoopForge: long-running agent operating system",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Initialize ~/.loopforge (config + database)
    Init,
    /// One-command onboarding check (init + config + doctor + optional first task)
    Onboard {
        /// Workspace directory for the first verification run
        #[arg(long, default_value = "loopforge-onboard-demo")]
        workspace: PathBuf,
        /// Optional explicit prompt for the first verification run
        #[arg(long)]
        prompt: Option<String>,
        /// Starter profile used when `--prompt` is not provided
        #[arg(long, value_enum, default_value_t = OnboardStarter::Hello)]
        starter: OnboardStarter,
        /// Skip running the first agent task and only run setup checks
        #[arg(long)]
        skip_agent: bool,
        /// Timeout for doctor probes (milliseconds)
        #[arg(long, default_value_t = 1500)]
        timeout_ms: u64,
    },
    /// Diagnose common setup issues (config, providers, browser, tooling)
    Doctor {
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
        /// Exit non-zero if any warnings are detected
        #[arg(long)]
        strict: bool,
        /// Timeout for network probes (milliseconds)
        #[arg(long, default_value_t = 1500)]
        timeout_ms: u64,
    },
    /// Run an agent session (LLM + tools + memory)
    Agent {
        #[command(subcommand)]
        command: AgentCommand,
    },
    /// Outbound channels (outbox + dispatcher)
    Channel {
        #[command(subcommand)]
        command: ChannelCommand,
    },
    /// ACP event/checkpoint inspection helpers
    Acp {
        #[command(subcommand)]
        command: AcpCommand,
    },
    /// Config helpers
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    /// Skills discovery, doctor and execution helpers
    Skills {
        #[command(subcommand)]
        command: SkillsCommand,
    },
    /// Long-running harness helpers (initializer + sessions)
    Harness {
        #[command(subcommand)]
        command: HarnessCommand,
    },
    /// Run LoopForge daemon (HTTP API)
    Daemon {
        #[command(subcommand)]
        command: DaemonCommand,
    },
    /// Release assistants (metadata + preflight checks)
    Release {
        #[command(subcommand)]
        command: ReleaseCommand,
    },
}

#[derive(Debug, clap::Subcommand)]
enum ConfigCommand {
    /// Validate ~/.loopforge/config.toml and exit non-zero when invalid
    Validate {
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, clap::Subcommand)]
enum SkillsCommand {
    /// List discovered skills (workspace + ~/.codex/skills)
    List {
        /// Workspace root directory
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
    /// Show one skill's resolved metadata
    Show {
        /// Skill name
        name: String,
        /// Workspace root directory
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
    /// Diagnose skill manifest and entry issues
    Doctor {
        /// Workspace root directory
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
        /// Exit non-zero on warnings too
        #[arg(long)]
        strict: bool,
    },
    /// Execute one skill with real runtime tools and model routing
    Run {
        /// Skill name
        name: String,
        /// Workspace root directory
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Input payload passed to the skill
        #[arg(long)]
        input: String,
        /// Optional session id (generated per-workspace if omitted)
        #[arg(long)]
        session: Option<String>,
        /// Task kind for model routing
        #[arg(long, value_enum, default_value_t = AgentKind::Coding)]
        kind: AgentKind,
    },
}

#[derive(Debug, clap::Subcommand)]
enum HarnessCommand {
    /// Initialize a workspace directory for long-running agent sessions
    Init {
        dir: PathBuf,
        /// Optional initializer prompt (generates a comprehensive features.json)
        #[arg(long)]
        prompt: Option<String>,
        /// Override session id (default: persisted per-workspace)
        #[arg(long)]
        session: Option<String>,
    },
    /// Run a harness session (preflight + agent run)
    Run {
        dir: PathBuf,
        /// User instruction for this session (if omitted, only runs preflight)
        #[arg(long)]
        prompt: Option<String>,
        /// Override session id (default: derived UUID per run)
        #[arg(long)]
        session: Option<String>,
        /// Max attempts when init.sh fails (default 3)
        #[arg(long, default_value_t = 3)]
        max_attempts: usize,
    },
}

#[derive(Debug, clap::Subcommand)]
enum AgentCommand {
    /// Run a single agent session in a workspace
    Run {
        /// Workspace root directory (tools are sandboxed to this path)
        #[arg(long)]
        workspace: PathBuf,
        /// User instruction for this run
        #[arg(long)]
        prompt: String,
        /// Optional system prompt (string)
        #[arg(long)]
        system: Option<String>,
        /// Optional session id (generated if omitted)
        #[arg(long)]
        session: Option<String>,
        /// Task kind for model routing
        #[arg(long, value_enum, default_value_t = AgentKind::Coding)]
        kind: AgentKind,
        /// Comma-separated allowed tool names for this session (session-level whitelist)
        #[arg(long, value_delimiter = ',')]
        allowed_tools: Vec<String>,
    },
}

#[derive(Debug, clap::Subcommand)]
enum ChannelCommand {
    /// Drain queued outbox messages once
    Drain {
        /// Max messages to attempt in one run
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    /// Run a long-lived worker that periodically drains the outbox
    Worker {
        /// Seconds between drain attempts
        #[arg(long, default_value_t = 5)]
        interval_secs: u64,
        /// Max messages to attempt per drain cycle
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
}

#[derive(Debug, clap::Subcommand)]
enum AcpCommand {
    /// List recent ACP events
    Events {
        /// Optional session id filter
        #[arg(long)]
        session: Option<String>,
        /// Max events to print
        #[arg(long, default_value_t = 100)]
        limit: usize,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
    /// Show ACP delivery checkpoints for one session
    Checkpoints {
        /// Session id
        #[arg(long)]
        session: String,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum AgentKind {
    Planning,
    Coding,
    Summary,
}

impl From<AgentKind> for rexos::router::TaskKind {
    fn from(value: AgentKind) -> Self {
        match value {
            AgentKind::Planning => rexos::router::TaskKind::Planning,
            AgentKind::Coding => rexos::router::TaskKind::Coding,
            AgentKind::Summary => rexos::router::TaskKind::Summary,
        }
    }
}

#[derive(Debug, clap::Subcommand)]
enum DaemonCommand {
    /// Start the daemon HTTP server
    Start {
        /// Listen address, e.g. 127.0.0.1:8787
        #[arg(long, default_value = "127.0.0.1:8787")]
        addr: String,
    },
}

#[derive(Debug, clap::Subcommand)]
enum ReleaseCommand {
    /// Check release metadata and preflight conditions
    Check {
        /// Release tag, e.g. v0.1.0 (defaults to v<workspace version>)
        #[arg(long)]
        tag: Option<String>,
        /// Repository root to check (default: current directory)
        #[arg(long, default_value = ".")]
        repo_root: PathBuf,
        /// Run `cargo test --workspace --locked` as part of preflight
        #[arg(long)]
        run_tests: bool,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => {
            let paths = RexosPaths::discover()?;
            paths.ensure_dirs()?;
            RexosConfig::ensure_default(&paths)?;
            MemoryStore::open_or_create(&paths)?;
            println!("Initialized {}", paths.base_dir.display());
        }
        Command::Onboard {
            workspace,
            prompt,
            starter,
            skip_agent,
            timeout_ms,
        } => {
            onboard::run(workspace, prompt, starter, skip_agent, timeout_ms).await?;
        }
        Command::Doctor {
            json,
            strict,
            timeout_ms,
        } => {
            let paths = RexosPaths::discover()?;
            let report = doctor::run_doctor(doctor::DoctorOptions {
                paths,
                timeout: std::time::Duration::from_millis(timeout_ms),
            })
            .await?;

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("{}", report.to_text());
            }

            let code = report.exit_code(strict);
            if code != 0 {
                std::process::exit(code);
            }
        }
        Command::Agent { command } => match command {
            AgentCommand::Run {
                workspace,
                prompt,
                system,
                session,
                kind,
                allowed_tools,
            } => {
                let paths = RexosPaths::discover()?;
                paths.ensure_dirs()?;
                RexosConfig::ensure_default(&paths)?;
                let cfg = RexosConfig::load(&paths)?;

                let memory = MemoryStore::open_or_create(&paths)?;
                let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg)?;
                let router = rexos::router::ModelRouter::new(cfg.router);
                let agent = rexos::agent::AgentRuntime::new(memory, llms, router);

                let session_id = match session {
                    Some(id) => id,
                    None => rexos::harness::resolve_session_id(&workspace)?,
                };
                if !allowed_tools.is_empty() {
                    agent.set_session_allowed_tools(&session_id, allowed_tools)?;
                }
                let out = agent
                    .run_session(
                        workspace,
                        &session_id,
                        system.as_deref(),
                        &prompt,
                        kind.into(),
                    )
                    .await?;
                println!("{out}");
                eprintln!("[loopforge] session_id={session_id}");
            }
        },
        Command::Channel { command } => match command {
            ChannelCommand::Drain { limit } => {
                let paths = RexosPaths::discover()?;
                paths.ensure_dirs()?;
                RexosConfig::ensure_default(&paths)?;
                MemoryStore::open_or_create(&paths)?;

                let dispatcher =
                    rexos::agent::OutboxDispatcher::new(MemoryStore::open_or_create(&paths)?)?;
                let summary = dispatcher.drain_once(limit).await?;
                println!("drain: sent={} failed={}", summary.sent, summary.failed);
            }
            ChannelCommand::Worker {
                interval_secs,
                limit,
            } => {
                let paths = RexosPaths::discover()?;
                paths.ensure_dirs()?;
                RexosConfig::ensure_default(&paths)?;
                MemoryStore::open_or_create(&paths)?;

                let dispatcher =
                    rexos::agent::OutboxDispatcher::new(MemoryStore::open_or_create(&paths)?)?;

                loop {
                    let summary = dispatcher.drain_once(limit).await?;
                    println!("drain: sent={} failed={}", summary.sent, summary.failed);
                    tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
                }
            }
        },
        Command::Acp { command } => match command {
            AcpCommand::Events {
                session,
                limit,
                json,
            } => {
                let paths = RexosPaths::discover()?;
                paths.ensure_dirs()?;
                RexosConfig::ensure_default(&paths)?;
                let memory = MemoryStore::open_or_create(&paths)?;

                let events = load_acp_events(&memory, session.as_deref(), limit)?;
                if json {
                    println!("{}", serde_json::to_string_pretty(&events)?);
                } else {
                    for ev in events {
                        let session = ev.get("session_id").and_then(|v| v.as_str()).unwrap_or("-");
                        let event_type = ev
                            .get("event_type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let created_at = ev.get("created_at").and_then(|v| v.as_i64()).unwrap_or(0);
                        println!("[{created_at}] session={session} type={event_type}");
                    }
                }
            }
            AcpCommand::Checkpoints { session, json } => {
                let paths = RexosPaths::discover()?;
                paths.ensure_dirs()?;
                RexosConfig::ensure_default(&paths)?;
                let memory = MemoryStore::open_or_create(&paths)?;

                let checkpoints = load_acp_checkpoints(&memory, &session)?;
                if json {
                    println!("{}", serde_json::to_string_pretty(&checkpoints)?);
                } else if checkpoints.is_empty() {
                    println!("no checkpoints for session {}", session);
                } else {
                    for cp in checkpoints {
                        let channel = cp.get("channel").and_then(|v| v.as_str()).unwrap_or("-");
                        let cursor = cp.get("cursor").and_then(|v| v.as_str()).unwrap_or("-");
                        let updated_at = cp.get("updated_at").and_then(|v| v.as_i64()).unwrap_or(0);
                        println!("[{updated_at}] channel={channel} cursor={cursor}");
                    }
                }
            }
        },
        Command::Config { command } => match command {
            ConfigCommand::Validate { json } => {
                let paths = RexosPaths::discover()?;
                let report = validate_config(&paths);
                if json {
                    println!("{}", serde_json::to_string_pretty(&report)?);
                } else if report.valid {
                    println!("config valid: {}", report.config_path);
                } else {
                    println!("config invalid: {}", report.config_path);
                    for err in &report.errors {
                        println!("- {err}");
                    }
                }

                if !report.valid {
                    std::process::exit(1);
                }
            }
        },
        Command::Skills { command } => match command {
            SkillsCommand::List { workspace, json } => {
                let list = skills::list_skills(&workspace)?;
                if json {
                    println!("{}", serde_json::to_string_pretty(&list)?);
                } else if list.is_empty() {
                    println!("no skills discovered");
                } else {
                    for item in list {
                        println!(
                            "{}  v{}  source={}  entry={}",
                            item.name, item.version, item.source, item.entry_path
                        );
                    }
                }
            }
            SkillsCommand::Show {
                name,
                workspace,
                json,
            } => {
                let skill = skills::find_skill(&workspace, &name)?;
                let item = serde_json::json!({
                    "name": skill.name,
                    "version": skill.manifest.version.to_string(),
                    "source": skills::source_name(skill.source),
                    "root_dir": skill.root_dir,
                    "manifest_path": skill.manifest_path,
                    "entry": skill.manifest.entry,
                    "permissions": skill.manifest.permissions,
                    "dependencies": skill
                        .manifest
                        .dependencies
                        .iter()
                        .map(|d| serde_json::json!({
                            "name": d.name,
                            "version_req": d.version_req.to_string(),
                        }))
                        .collect::<Vec<_>>(),
                });
                if json {
                    println!("{}", serde_json::to_string_pretty(&item)?);
                } else {
                    println!("{}", serde_json::to_string_pretty(&item)?);
                }
            }
            SkillsCommand::Doctor {
                workspace,
                json,
                strict,
            } => {
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
                    .any(|i| matches!(i.level, skills::SkillsDoctorLevel::Error));
                let has_warn = report
                    .issues
                    .iter()
                    .any(|i| matches!(i.level, skills::SkillsDoctorLevel::Warn));
                if has_error || (strict && has_warn) {
                    std::process::exit(1);
                }
            }
            SkillsCommand::Run {
                name,
                workspace,
                input,
                session,
                kind,
            } => {
                let paths = RexosPaths::discover()?;
                paths.ensure_dirs()?;
                RexosConfig::ensure_default(&paths)?;
                let cfg = RexosConfig::load(&paths)?;
                let skills_cfg = RexosConfig::load_skills_config(&paths).unwrap_or_default();

                std::fs::create_dir_all(&workspace)
                    .with_context(|| format!("create workspace: {}", workspace.display()))?;

                let skill = skills::find_skill(&workspace, &name)?;
                let skill_entry = skills::read_skill_entry(&skill)?;

                let memory = MemoryStore::open_or_create(&paths)?;
                let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg)?;
                let router = rexos::router::ModelRouter::new(cfg.router);
                let agent = rexos::agent::AgentRuntime::new(memory, llms, router);

                let session_id = match session {
                    Some(id) => id,
                    None => rexos::harness::resolve_session_id(&workspace)?,
                };
                let experimental_mode = skills_cfg.experimental;
                agent.set_session_skill_policy(
                    &session_id,
                    rexos::agent::SessionSkillPolicy {
                        allowlist: skills_cfg.allowlist,
                        require_approval: skills_cfg.require_approval,
                        auto_approve_readonly: skills_cfg.auto_approve_readonly,
                    },
                )?;
                if experimental_mode {
                    eprintln!("skills: experimental mode is enabled in config");
                }

                agent.record_skill_discovered(
                    &session_id,
                    &skill.name,
                    skills::source_name(skill.source),
                    &skill.manifest.version.to_string(),
                )?;
                agent.authorize_skill(&session_id, &skill.name, &skill.manifest.permissions)?;

                let allowed_tools = skills::permission_tools(&skill.manifest.permissions);
                if !allowed_tools.is_empty() {
                    agent.set_session_allowed_tools(&session_id, allowed_tools)?;
                }

                let system = format!(
                    "You are executing skill `{}` version {}.\\n\
Follow the skill instructions exactly.\\n\
If tool permissions are restricted, do not call tools outside the granted scope.\\n\\n\
--- SKILL INSTRUCTIONS START ---\\n{}\\n--- SKILL INSTRUCTIONS END ---",
                    skill.name, skill.manifest.version, skill_entry
                );

                let out = match agent
                    .run_session(workspace, &session_id, Some(&system), &input, kind.into())
                    .await
                {
                    Ok(out) => {
                        agent.record_skill_execution(
                            &session_id,
                            &skill.name,
                            &skill.manifest.permissions,
                            true,
                            None,
                        )?;
                        out
                    }
                    Err(e) => {
                        let err_text = e.to_string();
                        let _ = agent.record_skill_execution(
                            &session_id,
                            &skill.name,
                            &skill.manifest.permissions,
                            false,
                            Some(&err_text),
                        );
                        return Err(e);
                    }
                };

                println!("{out}");
                eprintln!("[loopforge] session_id={session_id}");
            }
        },
        Command::Harness { command } => match command {
            HarnessCommand::Init {
                dir,
                prompt,
                session,
            } => {
                if prompt.is_none() {
                    rexos::harness::init_workspace(&dir)?;
                    println!("Harness initialized in {}", dir.display());
                    return Ok(());
                }

                match rexos::harness::init_workspace(&dir) {
                    Ok(()) => {}
                    Err(e) => {
                        let msg = e.to_string();
                        if !msg.contains("already initialized") {
                            return Err(e);
                        }
                    }
                }

                let session_id = match session {
                    Some(s) => s,
                    None => rexos::harness::resolve_session_id(&dir)?,
                };

                let prompt = prompt.expect("checked above");

                let paths = RexosPaths::discover()?;
                paths.ensure_dirs()?;
                RexosConfig::ensure_default(&paths)?;
                let cfg = RexosConfig::load(&paths)?;

                let memory = MemoryStore::open_or_create(&paths)?;
                let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg)?;
                let router = rexos::router::ModelRouter::new(cfg.router);
                let agent = rexos::agent::AgentRuntime::new(memory, llms, router);

                rexos::harness::bootstrap_with_prompt(&agent, &dir, &session_id, &prompt).await?;

                println!("Harness bootstrapped in {}", dir.display());
                eprintln!("[loopforge] session_id={session_id}");
            }
            HarnessCommand::Run {
                dir,
                prompt,
                session,
                max_attempts,
            } => {
                if prompt.is_none() {
                    rexos::harness::preflight(&dir)?;
                    return Ok(());
                }

                let session_id = match session {
                    Some(s) => s,
                    None => rexos::harness::resolve_session_id(&dir)?,
                };

                let prompt = prompt.expect("checked above");

                let paths = RexosPaths::discover()?;
                paths.ensure_dirs()?;
                RexosConfig::ensure_default(&paths)?;
                let cfg = RexosConfig::load(&paths)?;

                let memory = MemoryStore::open_or_create(&paths)?;
                let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg)?;
                let router = rexos::router::ModelRouter::new(cfg.router);
                let agent = rexos::agent::AgentRuntime::new(memory, llms, router);

                let out =
                    rexos::harness::run_harness(&agent, &dir, &session_id, &prompt, max_attempts)
                        .await?;
                println!("{out}");
                eprintln!("[loopforge] session_id={session_id}");
            }
        },
        Command::Daemon { command } => match command {
            DaemonCommand::Start { addr } => {
                let addr = addr.parse()?;
                rexos::daemon::serve(addr).await?;
            }
        },
        Command::Release { command } => match command {
            ReleaseCommand::Check {
                tag,
                repo_root,
                run_tests,
                json,
            } => {
                let report = run_release_check(&repo_root, tag.as_deref(), run_tests)?;
                if json {
                    println!("{}", serde_json::to_string_pretty(&report)?);
                } else {
                    println!("{}", format_release_check_report(&report));
                }
                if !report.ok {
                    std::process::exit(1);
                }
            }
        },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_primary_name_is_loopforge() {
        use clap::CommandFactory;
        assert_eq!(Cli::command().get_name(), "loopforge");
    }

    #[test]
    fn cli_about_uses_loopforge_only_branding() {
        use clap::CommandFactory;
        let about = Cli::command()
            .get_about()
            .map(|s| s.to_string())
            .unwrap_or_default();
        assert!(
            about.contains("LoopForge"),
            "expected LoopForge about text, got: {about}"
        );
        assert!(
            !about.contains("RexOS"),
            "expected no RexOS mention, got: {about}"
        );
    }

    #[test]
    fn cli_parses_config_validate_with_loopforge_binary_name() {
        let parsed = Cli::try_parse_from(["loopforge", "config", "validate"]);
        assert!(
            parsed.is_ok(),
            "expected `loopforge config validate` to parse, got: {parsed:?}"
        );
    }

    #[test]
    fn cli_parses_release_check_subcommand() {
        let parsed = Cli::try_parse_from(["loopforge", "release", "check", "--tag", "v0.1.0"]);
        assert!(
            parsed.is_ok(),
            "expected `loopforge release check` to parse, got: {parsed:?}"
        );
    }

    #[test]
    fn cli_parses_acp_events_subcommand() {
        let parsed = Cli::try_parse_from([
            "loopforge",
            "acp",
            "events",
            "--session",
            "s-1",
            "--limit",
            "20",
        ]);
        assert!(
            parsed.is_ok(),
            "expected `loopforge acp events` to parse, got: {parsed:?}"
        );
    }

    #[test]
    fn cli_parses_acp_checkpoints_subcommand() {
        let parsed = Cli::try_parse_from(["loopforge", "acp", "checkpoints", "--session", "s-1"]);
        assert!(
            parsed.is_ok(),
            "expected `loopforge acp checkpoints` to parse, got: {parsed:?}"
        );
    }

    #[test]
    fn cli_parses_agent_run_allowed_tools() {
        let parsed = Cli::try_parse_from([
            "loopforge",
            "agent",
            "run",
            "--workspace",
            ".",
            "--prompt",
            "x",
            "--allowed-tools",
            "fs_read,web_fetch",
        ]);
        assert!(
            parsed.is_ok(),
            "expected agent run with --allowed-tools to parse, got: {parsed:?}"
        );
    }

    #[test]
    fn cli_parses_skills_list_subcommand() {
        let parsed = Cli::try_parse_from(["loopforge", "skills", "list", "--workspace", "."]);
        assert!(
            parsed.is_ok(),
            "expected `loopforge skills list` to parse, got: {parsed:?}"
        );
    }

    #[test]
    fn cli_parses_skills_run_subcommand() {
        let parsed = Cli::try_parse_from([
            "loopforge",
            "skills",
            "run",
            "hello-skill",
            "--workspace",
            ".",
            "--input",
            "do x",
        ]);
        assert!(
            parsed.is_ok(),
            "expected `loopforge skills run` to parse, got: {parsed:?}"
        );
    }

    #[test]
    fn cli_parses_onboard_subcommand() {
        let parsed = Cli::try_parse_from([
            "loopforge",
            "onboard",
            "--workspace",
            "loopforge-onboard-demo",
        ]);
        assert!(
            parsed.is_ok(),
            "expected `loopforge onboard` to parse, got: {parsed:?}"
        );
    }

    #[test]
    fn cli_parses_onboard_starter_profile() {
        let parsed = Cli::try_parse_from([
            "loopforge",
            "onboard",
            "--workspace",
            "loopforge-onboard-demo",
            "--starter",
            "workspace-brief",
        ]);
        assert!(
            parsed.is_ok(),
            "expected `loopforge onboard --starter workspace-brief` to parse, got: {parsed:?}"
        );
    }
}
