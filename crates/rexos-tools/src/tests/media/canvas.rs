use super::*;

#[tokio::test]
async fn canvas_present_writes_sanitized_html_file() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    let tools = Toolset::new(workspace.clone()).unwrap();
    let out = tools
        .call(
            "canvas_present",
            r#"{ "title": "Report", "html": "<h1>Hello</h1>" }"#,
        )
        .await
        .unwrap();

    let v: serde_json::Value = serde_json::from_str(&out).expect("canvas_present output is json");
    let saved_to = v
        .get("saved_to")
        .and_then(|v| v.as_str())
        .expect("saved_to");
    assert!(
        saved_to.ends_with(".html"),
        "unexpected saved_to: {saved_to}"
    );

    let html = std::fs::read_to_string(workspace.join(saved_to)).unwrap();
    assert!(html.contains("<h1>Hello</h1>"), "{html}");
    assert!(html.contains("<title>Report</title>"), "{html}");
}

#[tokio::test]
async fn canvas_present_rejects_dangerous_html() {
    let tmp = tempfile::tempdir().unwrap();
    let workspace = tmp.path().join("ws");
    std::fs::create_dir_all(&workspace).unwrap();

    let tools = Toolset::new(workspace).unwrap();

    let err = tools
        .call(
            "canvas_present",
            r#"{ "html": "<script>alert(1)</script>" }"#,
        )
        .await
        .unwrap_err();
    assert!(err.to_string().to_lowercase().contains("script"), "{err}");

    let err = tools
        .call(
            "canvas_present",
            r#"{ "html": "<img src=x onerror=alert(1)>" }"#,
        )
        .await
        .unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("event")
            || err.to_string().to_lowercase().contains("onerror")
            || err.to_string().to_lowercase().contains("handler"),
        "{err}"
    );

    let err = tools
        .call(
            "canvas_present",
            r#"{ "html": "<a href=\"javascript:alert(1)\">x</a>" }"#,
        )
        .await
        .unwrap_err();
    assert!(
        err.to_string().to_lowercase().contains("javascript"),
        "{err}"
    );
}
