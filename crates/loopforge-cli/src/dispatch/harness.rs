use crate::{cli::HarnessCommand, runtime_env};

pub(super) async fn run(command: HarnessCommand) -> anyhow::Result<()> {
    match command {
        HarnessCommand::Init {
            dir,
            prompt,
            session,
        } => {
            if prompt.is_none() {
                rexos::harness::init_workspace(&dir)?;
                println!("Harness initialized in {}", dir.display());
                return Ok(());
            }

            match rexos::harness::init_workspace(&dir) {
                Ok(()) => {}
                Err(err) => {
                    let msg = err.to_string();
                    if !msg.contains("already initialized") {
                        return Err(err);
                    }
                }
            }

            let session_id = match session {
                Some(id) => id,
                None => rexos::harness::resolve_session_id(&dir)?,
            };

            let prompt = prompt.expect("checked above");
            let (_paths, agent) = runtime_env::load_agent_runtime()?;

            rexos::harness::bootstrap_with_prompt(&agent, &dir, &session_id, &prompt).await?;

            println!("Harness bootstrapped in {}", dir.display());
            eprintln!("[loopforge] session_id={session_id}");
            Ok(())
        }
        HarnessCommand::Run {
            dir,
            prompt,
            session,
            max_attempts,
        } => {
            if prompt.is_none() {
                rexos::harness::preflight(&dir)?;
                return Ok(());
            }

            let session_id = match session {
                Some(id) => id,
                None => rexos::harness::resolve_session_id(&dir)?,
            };

            let prompt = prompt.expect("checked above");
            let (_paths, agent) = runtime_env::load_agent_runtime()?;

            let out = rexos::harness::run_harness(&agent, &dir, &session_id, &prompt, max_attempts)
                .await?;
            println!("{out}");
            eprintln!("[loopforge] session_id={session_id}");
            Ok(())
        }
    }
}
