use crate::{
    cli::ReleaseCommand,
    release_check::{format_release_check_report, run_release_check},
};

pub(super) fn run(command: ReleaseCommand) -> anyhow::Result<()> {
    match command {
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
            Ok(())
        }
    }
}
