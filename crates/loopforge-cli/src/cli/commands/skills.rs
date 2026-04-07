use std::path::PathBuf;

use super::agent::AgentKind;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub(crate) enum SkillsArchiveFormat {
    Auto,
    Zip,
    Tar,
    TarGz,
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum SkillsCommand {
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
    /// Install one skill archive from URL into workspace .loopforge/skills
    Install {
        /// Skill archive URL (zip/tar/tar.gz)
        url: String,
        /// Workspace root directory
        #[arg(long, default_value = ".")]
        workspace: PathBuf,
        /// Archive format (auto detects by bytes when omitted)
        #[arg(long, value_enum, default_value_t = SkillsArchiveFormat::Auto)]
        format: SkillsArchiveFormat,
        /// Replace an existing skill with the same manifest name
        #[arg(long)]
        force: bool,
        /// Print JSON output (machine-readable)
        #[arg(long)]
        json: bool,
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
