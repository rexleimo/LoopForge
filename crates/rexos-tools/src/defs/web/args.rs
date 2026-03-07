mod a2a;
mod fetch;
mod pdf;
mod search;
#[cfg(test)]
mod tests;

pub(crate) use a2a::{A2aDiscoverArgs, A2aSendArgs};
pub(crate) use fetch::WebFetchArgs;
pub(crate) use pdf::PdfArgs;
pub(crate) use search::WebSearchArgs;
