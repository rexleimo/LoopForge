mod chromium;
mod remote;
mod stderr;

pub(crate) use chromium::find_chromium;
pub(crate) use remote::{find_or_create_page_ws, pick_unused_port, validate_remote_cdp_base_url};
pub(crate) use stderr::read_devtools_url;
