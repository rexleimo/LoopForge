use crate::{cli::ChannelCommand, runtime_env};

pub(super) async fn run(command: ChannelCommand) -> anyhow::Result<()> {
    match command {
        ChannelCommand::Drain { limit } => {
            let dispatcher = runtime_env::load_dispatcher()?;
            let summary = dispatcher.drain_once(limit).await?;
            println!("drain: sent={} failed={}", summary.sent, summary.failed);
            Ok(())
        }
        ChannelCommand::Worker {
            interval_secs,
            limit,
        } => {
            let dispatcher = runtime_env::load_dispatcher()?;
            loop {
                let summary = dispatcher.drain_once(limit).await?;
                println!("drain: sent={} failed={}", summary.sent, summary.failed);
                tokio::time::sleep(std::time::Duration::from_secs(interval_secs)).await;
            }
        }
    }
}
