use anyhow::Context;
use tokio::io::AsyncWriteExt;

use crate::browser_runtime::PlaywrightBrowserSession;

pub(super) async fn write_bridge_command(
    session: &mut PlaywrightBrowserSession,
    cmd: &serde_json::Value,
) -> anyhow::Result<()> {
    let line = super::encode::encoded_bridge_command(cmd)?;

    tokio::time::timeout(super::super::shared::BRIDGE_TIMEOUT, async {
        session
            .stdin
            .write_all(line.as_bytes())
            .await
            .context("write bridge stdin")?;
        session.stdin.flush().await.context("flush bridge stdin")?;
        anyhow::Ok(())
    })
    .await
    .context("browser bridge timed out")?
}
