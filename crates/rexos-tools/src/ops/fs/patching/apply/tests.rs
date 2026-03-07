use crate::patch::PatchHunk;
use crate::Toolset;

use super::files::{ensure_parent_dir, rewrite_file};

#[test]
fn ensure_parent_dir_creates_missing_ancestors() {
    let tmp = tempfile::tempdir().unwrap();
    let dest = tmp.path().join("a/b/c.txt");

    ensure_parent_dir(&dest).unwrap();

    assert!(tmp.path().join("a/b").is_dir());
}

#[test]
fn rewrite_file_applies_hunks_to_existing_contents() {
    let tmp = tempfile::tempdir().unwrap();
    let file = tmp.path().join("note.txt");
    std::fs::write(&file, "alpha\nbeta\n").unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();

    rewrite_file(
        &tools,
        "note.txt",
        vec![PatchHunk {
            old_lines: vec!["beta".to_string()],
            new_lines: vec!["gamma".to_string()],
        }],
    )
    .unwrap();

    assert_eq!(std::fs::read_to_string(file).unwrap(), "alpha\ngamma\n");
}
