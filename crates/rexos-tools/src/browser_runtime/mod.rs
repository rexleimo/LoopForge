mod config;
mod playwright;
mod session;

pub(crate) use config::{browser_backend_default, browser_headless_default, BrowserBackend};
pub(crate) use playwright::PlaywrightBrowserSession;
pub(crate) use session::BrowserSession;
