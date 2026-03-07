use std::path::Path;

pub(super) const FS_READ_MAX_BYTES: u64 = 200_000;

pub(super) fn list_entry_suffix(is_dir: bool) -> &'static str {
    if is_dir {
        "/"
    } else {
        ""
    }
}

pub(super) fn write_success_output() -> &'static str {
    "ok"
}

pub(super) fn parent_dir(path: &Path) -> Option<&Path> {
    path.parent()
}
