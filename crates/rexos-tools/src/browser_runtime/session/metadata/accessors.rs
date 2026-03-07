use crate::browser_runtime::BrowserBackend;

use super::super::BrowserSession;

impl BrowserSession {
    pub(crate) fn backend(&self) -> BrowserBackend {
        match self {
            Self::Cdp(_) => BrowserBackend::Cdp,
            Self::Playwright(_) => BrowserBackend::Playwright,
        }
    }

    pub(crate) fn headless(&self) -> bool {
        match self {
            Self::Cdp(session) => session.headless,
            Self::Playwright(session) => session.headless,
        }
    }

    pub(crate) fn allow_private(&self) -> bool {
        match self {
            Self::Cdp(session) => session.allow_private,
            Self::Playwright(session) => session.allow_private,
        }
    }

    pub(crate) fn set_allow_private(&mut self, allow_private: bool) {
        match self {
            Self::Cdp(session) => session.allow_private = allow_private,
            Self::Playwright(session) => session.allow_private = allow_private,
        }
    }
}
