use anyhow::Context;

use super::super::super::CdpBrowserSession;
use super::super::shared::{connect_page_session, session_from_cdp};

impl CdpBrowserSession {
    pub(crate) async fn launch_local(
        http: reqwest::Client,
        headless: bool,
        allow_private: bool,
    ) -> anyhow::Result<Self> {
        let chrome_path = crate::browser_cdp::discovery::find_chromium()?;
        let port = crate::browser_cdp::discovery::pick_unused_port().context("pick unused port")?;

        let user_data_dir =
            std::env::temp_dir().join(format!("rexos-chrome-{}", uuid::Uuid::new_v4()));
        let args = super::command::chromium_args(port, &user_data_dir, headless);

        let mut child = super::command::chromium_command(&chrome_path, &args)
            .spawn()
            .with_context(|| format!("launch Chromium at {}", chrome_path.display()))?;

        let stderr = child.stderr.take().context("capture Chromium stderr")?;
        let _ws_url = crate::browser_cdp::discovery::read_devtools_url(stderr).await?;

        let base = reqwest::Url::parse(&format!("http://127.0.0.1:{port}"))
            .context("parse local CDP base url")?;
        let cdp = connect_page_session(&http, &base).await?;

        Ok(session_from_cdp(
            headless,
            allow_private,
            Some(child),
            Some(user_data_dir),
            cdp,
        ))
    }
}
