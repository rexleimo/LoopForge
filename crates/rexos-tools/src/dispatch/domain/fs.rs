pub(super) fn is_fs_tool(name: &str) -> bool {
    matches!(
        name,
        "fs_read" | "file_read" | "fs_write" | "file_write" | "file_list" | "apply_patch"
    )
}
