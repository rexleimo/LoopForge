use anyhow::{bail, Context};

use crate::Toolset;

impl Toolset {
    pub(crate) async fn browser_back(&self) -> anyhow::Result<String> {
        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context("browser session not started; call browser_navigate first")?;
        let out = session.back().await?;
        super::super::ensure_output_url_allowed(session, &out).await?;
        Ok(out.to_string())
    }

    pub(crate) async fn browser_scroll(
        &self,
        direction: Option<&str>,
        amount: Option<i64>,
    ) -> anyhow::Result<String> {
        let direction = direction.unwrap_or("down").trim().to_ascii_lowercase();
        let amount = amount.unwrap_or(600).clamp(0, 50_000);
        if !matches!(direction.as_str(), "down" | "up" | "left" | "right") {
            bail!("invalid direction: {direction} (expected down/up/left/right)");
        }

        let mut guard = self.browser.lock().await;
        let session = guard
            .as_mut()
            .context("browser session not started; call browser_navigate first")?;
        let out = session.scroll(&direction, amount).await?;
        Ok(out.to_string())
    }
}
