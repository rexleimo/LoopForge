use anyhow::Context;
use tokio::io::BufReader;

use crate::browser_runtime::PlaywrightBrowserSession;

impl PlaywrightBrowserSession {
    pub(crate) async fn spawn(headless: bool, allow_private: bool) -> anyhow::Result<Self> {
        let mut child = super::command::spawn_bridge_process(headless)?;
        let stdin = child.stdin.take().context("capture bridge stdin")?;
        let stdout = child.stdout.take().context("capture bridge stdout")?;

        let mut session = Self {
            headless,
            allow_private,
            child,
            stdin,
            stdout: BufReader::new(stdout),
        };

        let ready = session.read_response().await.context("bridge ready")?;
        let _ = ready.into_data()?;

        Ok(session)
    }
}
