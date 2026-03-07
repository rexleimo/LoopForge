use tokio::io::BufReader;

mod bridge;
mod session;

pub(crate) use bridge::BridgeResponse;

pub(crate) struct PlaywrightBrowserSession {
    pub(crate) headless: bool,
    pub(crate) allow_private: bool,
    child: tokio::process::Child,
    stdin: tokio::process::ChildStdin,
    stdout: BufReader<tokio::process::ChildStdout>,
}

impl std::fmt::Debug for PlaywrightBrowserSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlaywrightBrowserSession")
            .field("headless", &self.headless)
            .field("allow_private", &self.allow_private)
            .finish_non_exhaustive()
    }
}

impl Drop for PlaywrightBrowserSession {
    fn drop(&mut self) {
        let _ = self.child.start_kill();
    }
}
