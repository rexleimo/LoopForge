mod navigation;
mod page;

use crate::browser_runtime::BrowserSession;
use crate::defs::ensure_browser_url_allowed;

async fn ensure_output_url_allowed(
    session: &BrowserSession,
    out: &serde_json::Value,
) -> anyhow::Result<()> {
    if let Some(url) = out.get("url").and_then(|v| v.as_str()) {
        ensure_browser_url_allowed(url, session.allow_private()).await?;
    }
    Ok(())
}
