use crate::browser_cdp;

use super::PlaywrightBrowserSession;

mod commands;
mod metadata;

pub(crate) enum BrowserSession {
    Cdp(browser_cdp::CdpBrowserSession),
    Playwright(PlaywrightBrowserSession),
}
