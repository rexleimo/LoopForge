use rexos::{config::RexosConfig, memory::MemoryStore};

use super::flow_types::PreparedOnboard;
use super::outcome::{emit_onboard_report, record_and_print_onboard_attempt};
use super::runtime::maybe_select_ollama_model;
use super::starter::{recommended_next_command, recommended_retry_command, shell_quote};
use super::{classify_onboard_failure, verify_onboard_artifact, OnboardTaskReport};

pub(super) fn emit_skipped_report(prepared: &PreparedOnboard) -> anyhow::Result<()> {
    let report = prepared.report_base.build_report(
        OnboardTaskReport {
            status: "skipped".to_string(),
            session_id: None,
            failure_category: None,
            error: None,
        },
        format!(
            "loopforge onboard --workspace {} --starter {}",
            shell_quote(&prepared.workspace.display().to_string()),
            prepared.starter.as_str()
        ),
    );
    emit_onboard_report(&prepared.workspace, &report)
}

pub(super) async fn run_first_agent_task(
    prepared: PreparedOnboard,
    prompt: Option<&str>,
    timeout_ms: u64,
) -> anyhow::Result<()> {
    let mut cfg = RexosConfig::load(&prepared.paths)?;
    maybe_select_ollama_model(&mut cfg, timeout_ms).await;

    let memory = MemoryStore::open_or_create(&prepared.paths)?;
    let llms = rexos::llm::registry::LlmRegistry::from_config(&cfg)?;
    let security = cfg.security.clone();
    let router = rexos::router::ModelRouter::new(cfg.router);
    let agent =
        rexos::agent::AgentRuntime::new_with_security_config(memory, llms, router, security);

    let session_id = rexos::harness::resolve_session_id(&prepared.workspace)?;
    let out = match agent
        .run_session(
            prepared.workspace.clone(),
            &session_id,
            None,
            &prepared.effective_prompt,
            rexos::router::TaskKind::Coding,
        )
        .await
    {
        Ok(out) => out,
        Err(err) => {
            let err_msg = err.to_string();
            let failure_category = classify_onboard_failure(&err_msg);
            record_and_print_onboard_attempt(
                &prepared.paths,
                &prepared.workspace,
                &session_id,
                false,
                Some(&failure_category),
                Some(&err_msg),
            );
            let report = prepared.report_base.build_report(
                OnboardTaskReport {
                    status: "failed".to_string(),
                    session_id: Some(session_id.clone()),
                    failure_category: Some(failure_category.clone()),
                    error: Some(err_msg.clone()),
                },
                recommended_next_command(&prepared.workspace, false),
            );
            emit_onboard_report(&prepared.workspace, &report)?;
            eprintln!("onboard: first agent run failed: {err}");
            eprintln!(
                "hint: run `ollama list` and set [providers.ollama].default_model in ~/.loopforge/config.toml to an available chat model"
            );
            return Err(err);
        }
    };
    println!("{out}");
    eprintln!("[loopforge] session_id={session_id}");

    if let Err(err) = verify_onboard_artifact(&prepared.workspace, prompt, prepared.starter) {
        let err_msg = err.to_string();
        let failure_category = "expected_artifact_missing".to_string();
        record_and_print_onboard_attempt(
            &prepared.paths,
            &prepared.workspace,
            &session_id,
            false,
            Some(&failure_category),
            Some(&err_msg),
        );
        let report = prepared.report_base.build_report(
            OnboardTaskReport {
                status: "failed".to_string(),
                session_id: Some(session_id.clone()),
                failure_category: Some(failure_category.clone()),
                error: Some(err_msg.clone()),
            },
            recommended_retry_command(&prepared.workspace, &session_id, &prepared.effective_prompt),
        );
        emit_onboard_report(&prepared.workspace, &report)?;
        eprintln!("onboard: first agent run finished without the expected starter artifact");
        return Err(err);
    }

    record_and_print_onboard_attempt(
        &prepared.paths,
        &prepared.workspace,
        &session_id,
        true,
        None,
        None,
    );
    let report = prepared.report_base.build_report(
        OnboardTaskReport {
            status: "succeeded".to_string(),
            session_id: Some(session_id.clone()),
            failure_category: None,
            error: None,
        },
        recommended_next_command(&prepared.workspace, true),
    );
    emit_onboard_report(&prepared.workspace, &report)?;
    println!("onboard done (first agent run completed)");
    Ok(())
}
