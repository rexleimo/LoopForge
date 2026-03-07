mod encode;
mod send;
#[cfg(test)]
mod tests;

use crate::browser_runtime::PlaywrightBrowserSession;

#[cfg(test)]
pub(super) fn encoded_bridge_command(cmd: &serde_json::Value) -> anyhow::Result<String> {
    encode::encoded_bridge_command(cmd)
}

impl PlaywrightBrowserSession {
    pub(crate) async fn send(
        &mut self,
        cmd: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        send::write_bridge_command(self, &cmd).await?;
        self.read_response().await?.into_data()
    }
}
