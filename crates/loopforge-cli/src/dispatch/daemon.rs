use crate::cli::DaemonCommand;

pub(super) async fn run(command: DaemonCommand) -> anyhow::Result<()> {
    match command {
        DaemonCommand::Start { addr } => {
            let addr = addr.parse()?;
            rexos::daemon::serve(addr).await?;
            Ok(())
        }
    }
}
