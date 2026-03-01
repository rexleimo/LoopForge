use clap::Parser;

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
    }

    Ok(())
}

