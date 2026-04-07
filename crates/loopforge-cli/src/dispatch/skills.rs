mod doctor;
mod install;
mod listing;
mod run;

use crate::cli::SkillsCommand;

pub(super) async fn run(command: SkillsCommand) -> anyhow::Result<()> {
    match command {
        SkillsCommand::List { workspace, json } => listing::run_list(workspace, json),
        SkillsCommand::Show {
            name,
            workspace,
            json,
        } => listing::run_show(name, workspace, json),
        SkillsCommand::Doctor {
            workspace,
            json,
            strict,
        } => doctor::run_doctor(workspace, json, strict),
        SkillsCommand::Install {
            url,
            workspace,
            format,
            force,
            json,
        } => install::run_install(url, workspace, format, force, json).await,
        SkillsCommand::Run {
            name,
            workspace,
            input,
            session,
            kind,
        } => run::run_skill(name, workspace, input, session, kind).await,
    }
}
