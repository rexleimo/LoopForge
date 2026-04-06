use crate::cli::McpCommand;

mod diagnose;
mod json;
#[cfg(test)]
mod tests;
mod tools;

pub(super) async fn run(command: McpCommand) -> anyhow::Result<()> {
    match command {
        McpCommand::Diagnose {
            workspace,
            session,
            config,
            resources,
            prompts,
            json,
        } => diagnose::run_diagnose(workspace, session, config, resources, prompts, json).await,
    }
}
