use clap::Parser;

mod acp;
mod cli;
mod config_validation;
mod dispatch;
mod doctor;
mod onboard;
mod release_check;
mod runtime_env;
mod skills;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dispatch::run(cli::Cli::parse()).await
}
