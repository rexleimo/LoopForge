use anyhow::{bail, Context};
use tokio::io::AsyncBufReadExt;

use crate::browser_runtime::playwright::BridgeResponse;
use crate::browser_runtime::PlaywrightBrowserSession;

pub(super) async fn read_response(
    session: &mut PlaywrightBrowserSession,
) -> anyhow::Result<BridgeResponse> {
    let mut line = String::new();
    let n = tokio::time::timeout(
        super::super::shared::BRIDGE_TIMEOUT,
        session.stdout.read_line(&mut line),
    )
    .await
    .context("browser bridge timed out")?
    .context("read bridge stdout")?;

    if n == 0 {
        bail!("browser bridge closed unexpectedly");
    }

    serde_json::from_str(super::parse::trimmed_bridge_response_line(&line))
        .context("parse bridge response")
}
