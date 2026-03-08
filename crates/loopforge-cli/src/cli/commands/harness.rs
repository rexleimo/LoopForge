use std::path::PathBuf;

#[derive(Debug, clap::Subcommand)]
pub(crate) enum HarnessCommand {
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
