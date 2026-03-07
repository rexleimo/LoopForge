use std::path::Path;

use super::{list_entry_suffix, parent_dir, write_success_output};

#[test]
fn list_entry_suffix_marks_directories_only() {
    assert_eq!(list_entry_suffix(true), "/");
    assert_eq!(list_entry_suffix(false), "");
}

#[test]
fn write_success_output_stays_ok() {
    assert_eq!(write_success_output(), "ok");
    assert!(parent_dir(Path::new("nested/a.txt")).is_some());
}
