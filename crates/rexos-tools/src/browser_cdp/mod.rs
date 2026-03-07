mod connection;
mod discovery;
mod session;

#[cfg(test)]
mod tests;

pub use session::CdpBrowserSession;

pub(super) const CDP_CONNECT_TIMEOUT_SECS: u64 = 15;
pub(super) const CDP_COMMAND_TIMEOUT_SECS: u64 = 30;
pub(super) const PAGE_LOAD_POLL_INTERVAL_MS: u64 = 200;
pub(super) const PAGE_LOAD_MAX_POLLS: u32 = 150;
