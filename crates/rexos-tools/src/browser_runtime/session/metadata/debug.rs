use super::super::BrowserSession;

impl std::fmt::Debug for BrowserSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrowserSession")
            .field("backend", &super::backend_label(self.backend()))
            .field("headless", &self.headless())
            .field("allow_private", &self.allow_private())
            .finish()
    }
}
