use super::super::PlaywrightBrowserSession;

impl PlaywrightBrowserSession {
    pub(crate) async fn kill(&mut self) {
        let _ = self.child.kill().await;
        let _ = self.child.wait().await;
    }
}
