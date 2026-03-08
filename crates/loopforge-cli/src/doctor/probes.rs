mod browser;
mod config;
mod system;

use anyhow::Context;

use super::actions::{derive_next_actions, summarize};
use super::{DoctorOptions, DoctorReport};

pub async fn run_doctor(opts: DoctorOptions) -> anyhow::Result<DoctorReport> {
    let http = reqwest::Client::builder()
        .timeout(opts.timeout)
        .build()
        .context("build http client")?;

    let mut checks = Vec::new();
    let cfg = config::load_config_checks(&mut checks, &opts);

    if let Some(cfg) = cfg.as_ref() {
        config::push_runtime_checks(&mut checks, cfg, &http).await;
    }

    browser::push_browser_checks(&mut checks, &http).await;
    system::push_command_checks(&mut checks);

    let summary = summarize(&checks);
    let next_actions = derive_next_actions(&checks);
    Ok(DoctorReport {
        checks,
        summary,
        next_actions,
    })
}
