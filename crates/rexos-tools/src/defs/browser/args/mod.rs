mod capture;
mod interaction;
mod navigation;
mod wait;

pub(crate) use capture::BrowserScreenshotArgs;
pub(crate) use interaction::{
    BrowserClickArgs, BrowserPressKeyArgs, BrowserRunJsArgs, BrowserTypeArgs,
};
pub(crate) use navigation::{BrowserNavigateArgs, BrowserScrollArgs};
pub(crate) use wait::{BrowserWaitArgs, BrowserWaitForArgs};
