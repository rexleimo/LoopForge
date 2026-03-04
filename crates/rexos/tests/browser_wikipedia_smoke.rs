use rexos::llm::openai_compat::{ChatCompletionRequest, ChatMessage, OpenAiCompatibleClient, Role};
use rexos::tools::Toolset;

#[tokio::test]
#[ignore]
async fn browser_wikipedia_and_summarize_with_ollama_smoke() {
    let model = std::env::var("REXOS_OLLAMA_MODEL").unwrap_or_else(|_| "qwen3:4b".to_string());
    let keep_workspace_dir = std::env::var("REXOS_BROWSER_SMOKE_WORKSPACE")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    let keep_artifacts = keep_workspace_dir.is_some();
    let headless = match std::env::var("REXOS_BROWSER_HEADLESS") {
        Ok(v) => match v.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => true,
            "0" | "false" | "no" | "off" => false,
            _ => true,
        },
        Err(_) => true,
    };

    let (tmp, workspace) = match keep_workspace_dir.as_deref() {
        Some(dir) => {
            let p = std::path::PathBuf::from(dir);
            std::fs::create_dir_all(&p).expect("create REXOS_BROWSER_SMOKE_WORKSPACE");
            println!("[rexos][wikipedia_smoke] artifacts_dir={}", p.display());
            (None, p)
        }
        None => {
            let tmp = tempfile::tempdir().unwrap();
            let workspace = tmp.path().to_path_buf();
            (Some(tmp), workspace)
        }
    };
    let _tmp_guard = tmp;
    let tools = Toolset::new(workspace.clone()).expect("create toolset");

    let nav = tools
        .call(
            "browser_navigate",
            &serde_json::json!({
                "url": "https://www.wikipedia.org",
                "timeout_ms": 30000,
                "headless": headless,
            })
            .to_string(),
        )
        .await
        .expect("browser_navigate wikipedia");
    let nav_v: serde_json::Value = serde_json::from_str(&nav).unwrap();
    let title = nav_v.get("title").and_then(|v| v.as_str()).unwrap_or("");
    assert!(
        title.to_ascii_lowercase().contains("wikipedia"),
        "unexpected wikipedia title: {title:?}"
    );

    let _ = tools
        .call(
            "browser_screenshot",
            r#"{ "path": ".rexos/browser/wikipedia_home.png" }"#,
        )
        .await
        .expect("browser_screenshot wikipedia home");

    let _ = tools
        .call(
            "browser_wait_for",
            r#"{ "selector": "body", "timeout_ms": 10000 }"#,
        )
        .await;

    let page = tools
        .call("browser_read_page", r#"{}"#)
        .await
        .expect("browser_read_page wikipedia");
    let page_v: serde_json::Value = serde_json::from_str(&page).unwrap();
    let page_url = page_v
        .get("url")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let page_text = page_v
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    assert!(
        page_url.to_ascii_lowercase().contains("wikipedia.org"),
        "unexpected wikipedia url: {page_url:?}"
    );
    assert!(
        page_text.chars().count() >= 500,
        "expected wikipedia page text >= 500 chars (got {})",
        page_text.len()
    );

    let screenshot_path = workspace.join(".rexos/browser/wikipedia_home.png");
    let screenshot_bytes = std::fs::read(&screenshot_path).expect("read screenshot");
    assert!(
        screenshot_bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]),
        "screenshot is not PNG: {}",
        screenshot_path.display()
    );

    let _ = tools.call("browser_close", r#"{}"#).await;

    let client = OpenAiCompatibleClient::new("http://127.0.0.1:11434/v1".to_string(), None)
        .expect("create ollama client");
    let sys = ChatMessage {
        role: Role::System,
        content: Some(
            "Summarize key facts from page text. If uncertain, say uncertainty explicitly."
                .to_string(),
        ),
        name: None,
        tool_call_id: None,
        tool_calls: None,
    };
    let user = ChatMessage {
        role: Role::User,
        content: Some(format!(
            "URL: {page_url}\n\nText:\n{}\n\nTask: reply in Chinese with 3 concise bullets.",
            page_text.chars().take(8000).collect::<String>()
        )),
        name: None,
        tool_call_id: None,
        tool_calls: None,
    };
    let res = client
        .chat_completions(ChatCompletionRequest {
            model,
            messages: vec![sys, user],
            tools: vec![],
            temperature: Some(0.0),
        })
        .await
        .expect("ollama summarize wikipedia");
    let summary = res.content.unwrap_or_default();
    println!("[rexos][wikipedia_smoke] url={page_url}");
    println!("[rexos][wikipedia_smoke] summary={summary}");
    assert!(!summary.trim().is_empty(), "empty summary");

    if keep_artifacts {
        let notes = workspace.join("notes");
        std::fs::create_dir_all(&notes).expect("create notes dir");
        std::fs::write(notes.join("wikipedia_summary.md"), format!("{summary}\n"))
            .expect("write wikipedia_summary.md");
    }
}
