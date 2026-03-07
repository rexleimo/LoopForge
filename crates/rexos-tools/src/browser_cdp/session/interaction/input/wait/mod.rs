mod matchers;
mod polling;
mod result;
#[cfg(test)]
mod tests;
mod validate;

use std::time::Duration;

use anyhow::bail;
use serde_json::Value;

use self::matchers::waited_for_matches;
use self::result::{wait_response, wait_satisfied};
use super::super::super::CdpBrowserSession;

impl CdpBrowserSession {
    pub async fn wait_for(
        &self,
        selector: Option<&str>,
        text: Option<&str>,
        timeout_ms: Option<u64>,
    ) -> anyhow::Result<Value> {
        let (selector, text) = validate::normalized_wait_args(selector, text);
        if selector.is_none() && text.is_none() {
            bail!("wait_for requires selector or text");
        }

        let max_ms = validate::clamped_timeout_ms(timeout_ms);
        let poll_interval_ms = super::super::super::super::PAGE_LOAD_POLL_INTERVAL_MS;

        for _ in 0..polling::poll_count(max_ms, poll_interval_ms) {
            let waited_for = waited_for_matches(self, selector, text).await;
            if wait_satisfied(&waited_for, selector, text) {
                return Ok(wait_response(self, waited_for, max_ms).await);
            }

            tokio::time::sleep(Duration::from_millis(poll_interval_ms)).await;
        }

        bail!("timed out waiting ({}ms)", max_ms);
    }
}
