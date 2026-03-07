mod list;
mod read;
mod shared;
#[cfg(test)]
mod tests;
mod write;

use std::path::Path;

fn list_entry_suffix(is_dir: bool) -> &'static str {
    shared::list_entry_suffix(is_dir)
}

fn write_success_output() -> &'static str {
    shared::write_success_output()
}

fn parent_dir(path: &Path) -> Option<&Path> {
    shared::parent_dir(path)
}
