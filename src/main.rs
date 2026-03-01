use clap::Parser;
use std::path::PathBuf;

use rexos::{config::RexosConfig, memory::MemoryStore, paths::RexosPaths};

#[derive(Debug, Parser)]
#[command(name = "rexos")]
#[command(about = "RexOS: long-running agent operating system", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    /// Initialize ~/.rexos (config + database)
    Init,
    /// Run an agent session (LLM + tools + memory)
    Agent {
        #[command(subcommand)]
        command: AgentCommand,
    },
    /// Long-running harness helpers (initializer + sessions)
    Harness {
        #[command(subcommand)]
        command: HarnessCommand,
    },
    /// Run RexOS daemon (HTTP API)
    Daemon {
        #[command(subcommand)]
        command: DaemonCommand,
    },
}

#[derive(Debug, clap::Subcommand)]
enum HarnessCommand {
    /// Initialize a workspace directory for long-running agent sessions
    Init { dir: PathBuf },
    /// Run a harness session (preflight + agent run)
    Run {
        dir: PathBuf,
        /// User instruction for this session (if omitted, only runs preflight)
        #[arg(long)]
        prompt: Option<String>,
        /// Override session id (default: derived UUID per run)
        #[arg(long)]
        session: Option<String>,
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
        Command::Agent { command } => match command {
            AgentCommand::Run {
                workspace,
                prompt,
                system,
                session,
                kind,
            } => {
                let paths = RexosPaths::discover()?;
                paths.ensure_dirs()?;
                RexosConfig::ensure_default(&paths)?;
                let cfg = RexosConfig::load(&paths)?;

                let memory = MemoryStore::open_or_create(&paths)?;
                let api_key = cfg.api_key();
                let llm = rexos::llm::openai_compat::OpenAiCompatibleClient::new(
                    cfg.llm.base_url,
                    api_key,
                )?;
                let router = rexos::router::ModelRouter::new(cfg.router);
                let agent = rexos::agent::AgentRuntime::new(memory, llm, router);

                let session_id = session.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
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
                eprintln!("[rexos] session_id={session_id}");
            }
        },
        Command::Harness { command } => match command {
            HarnessCommand::Init { dir } => {
                rexos::harness::init_workspace(&dir)?;
                println!("Harness initialized in {}", dir.display());
            }
            HarnessCommand::Run {
                dir,
                prompt,
                session,
            } => {
                rexos::harness::preflight(&dir)?;
                if let Some(prompt) = prompt {
                    let paths = RexosPaths::discover()?;
                    paths.ensure_dirs()?;
                    RexosConfig::ensure_default(&paths)?;
                    let cfg = RexosConfig::load(&paths)?;

                    let memory = MemoryStore::open_or_create(&paths)?;
                    let api_key = cfg.api_key();
                    let llm = rexos::llm::openai_compat::OpenAiCompatibleClient::new(
                        cfg.llm.base_url,
                        api_key,
                    )?;
                    let router = rexos::router::ModelRouter::new(cfg.router);
                    let agent = rexos::agent::AgentRuntime::new(memory, llm, router);

                    let session_id =
                        session.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

                    let harness_system = r#"You are RexOS running a long-horizon harness session.

Rules:
- Work only inside the workspace directory.
- Make small, incremental progress.
- Prefer using tools (`fs_read`, `fs_write`, `shell`) to inspect and change files.
- If you change code, run the workspace's `init.sh` (smoke checks) and fix any failures.
- Append a short summary to `rexos-progress.md`.
- Commit meaningful progress to git with a descriptive message.
"#;

                    let out = agent
                        .run_session(
                            dir,
                            &session_id,
                            Some(harness_system),
                            &prompt,
                            rexos::router::TaskKind::Coding,
                        )
                        .await?;
                    println!("{out}");
                    eprintln!("[rexos] session_id={session_id}");
                }
            }
        },
        Command::Daemon { command } => match command {
            DaemonCommand::Start { addr } => {
                let addr = addr.parse()?;
                rexos::daemon::serve(addr).await?;
            }
        },
    }

    Ok(())
}
