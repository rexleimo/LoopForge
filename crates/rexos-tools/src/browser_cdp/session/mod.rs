use std::path::PathBuf;

use super::connection::CdpConnection;

mod helpers;
mod interaction;
mod launch;
mod navigation;

pub struct CdpBrowserSession {
    pub headless: bool,
    pub allow_private: bool,
    process: Option<tokio::process::Child>,
    user_data_dir: Option<PathBuf>,
    cdp: CdpConnection,
}

impl CdpBrowserSession {
    pub async fn close(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
        if let Some(dir) = self.user_data_dir.take() {
            let _ = std::fs::remove_dir_all(dir);
        }
    }
}

impl Drop for CdpBrowserSession {
    fn drop(&mut self) {
        if let Some(child) = &mut self.process {
            let _ = child.start_kill();
        }
        if let Some(dir) = self.user_data_dir.take() {
            let _ = std::fs::remove_dir_all(dir);
        }
    }
}
