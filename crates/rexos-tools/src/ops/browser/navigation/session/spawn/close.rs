use crate::Toolset;

impl Toolset {
    pub(crate) async fn browser_close(&self) -> anyhow::Result<String> {
        let mut guard = self.browser.lock().await;
        if let Some(mut session) = guard.take() {
            session.close().await;
        }
        Ok("ok".to_string())
    }
}
