mod accessors;
mod debug;
mod shared;
#[cfg(test)]
mod tests;

use crate::browser_runtime::BrowserBackend;

fn backend_label(backend: BrowserBackend) -> &'static str {
    shared::backend_label(backend)
}
