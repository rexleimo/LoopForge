use super::*;

#[test]
fn validate_relative_path_rejects_parent_and_absolute() {
    assert!(validate_relative_path("../a").is_err());
    assert!(validate_relative_path("/etc/passwd").is_err());
    assert!(validate_relative_path("").is_err());
    assert!(validate_relative_path(".").is_err());
    assert!(validate_relative_path("./../a").is_err());
}

#[tokio::test]
async fn core_file_tools_work_via_primary_names() {
    let tmp = tempfile::tempdir().unwrap();
    let tools = Toolset::new(tmp.path().to_path_buf()).unwrap();

    tools
        .call(
            "fs_write",
            r#"{ "path": "nested/a.txt", "content": "hello core" }"#,
        )
        .await
        .unwrap();

    let out = tools
        .call("fs_read", r#"{ "path": "nested/a.txt" }"#)
        .await
        .unwrap();
    assert_eq!(out, "hello core");

    let listing = tools
        .call("file_list", r#"{ "path": "nested" }"#)
        .await
        .unwrap();
    assert_eq!(listing, "a.txt");
}

#[tokio::test]
async fn compat_file_tools_work_via_aliases() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    let tools = Toolset::new(workspace.clone()).unwrap();

    tools
        .call("file_write", r#"{ "path": "a.txt", "content": "hello" }"#)
        .await
        .unwrap();

    let content = tools
        .call("file_read", r#"{ "path": "a.txt" }"#)
        .await
        .unwrap();
    assert_eq!(content, "hello");

    std::fs::create_dir_all(workspace.join("dir")).unwrap();
    std::fs::write(workspace.join("dir").join("b.txt"), "world").unwrap();

    let listing = tools.call("file_list", r#"{ "path": "." }"#).await.unwrap();
    assert!(listing.contains("a.txt"), "{listing}");
    assert!(
        listing.contains("dir/") || listing.contains("dir"),
        "{listing}"
    );
}

#[tokio::test]
async fn compat_apply_patch_adds_and_updates_files() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();
    let tools = Toolset::new(workspace.clone()).unwrap();

    let patch = r#"*** Begin Patch
*** Add File: greet.txt
+hi
*** Update File: greet.txt
@@
-hi
+hello
*** End Patch"#;

    let _ = tools
        .call(
            "apply_patch",
            &format!(
                r#"{{ "patch": {} }}"#,
                serde_json::to_string(patch).unwrap()
            ),
        )
        .await
        .unwrap();

    let content = std::fs::read_to_string(workspace.join("greet.txt")).unwrap();
    assert_eq!(content.trim_end(), "hello");
}
