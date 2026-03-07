mod parse;
mod receive;
#[cfg(test)]
mod tests;

use crate::browser_runtime::playwright::BridgeResponse;
use crate::browser_runtime::PlaywrightBrowserSession;

#[cfg(test)]
pub(super) fn trimmed_bridge_response_line(line: &str) -> &str {
    parse::trimmed_bridge_response_line(line)
}

impl PlaywrightBrowserSession {
    pub(crate) async fn read_response(&mut self) -> anyhow::Result<BridgeResponse> {
        receive::read_response(self).await
    }
}
