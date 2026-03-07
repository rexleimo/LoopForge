use anyhow::bail;

use crate::browser_runtime::{BrowserBackend, BrowserSession};

pub(super) fn ensure_session_compatible(
    session: &Option<BrowserSession>,
    backend: BrowserBackend,
    headless: Option<bool>,
) -> anyhow::Result<()> {
    let Some(session) = session.as_ref() else {
        return Ok(());
    };

    if session.backend() != backend {
        bail!(
            "browser session already started with backend={:?}; call browser_close before switching to backend={:?}",
            session.backend(),
            backend
        );
    }

    if let Some(requested) = headless {
        let session_headless = session.headless();
        if session_headless != requested {
            bail!(
                "browser session already started with headless={session_headless}; call browser_close before starting a new session with headless={requested}"
            );
        }
    }

    Ok(())
}
