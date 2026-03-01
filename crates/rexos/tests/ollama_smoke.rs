#[tokio::test]
#[ignore]
async fn ollama_openai_compat_smoke() {
    let model = std::env::var("REXOS_OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".to_string());
    let client = rexos::llm::openai_compat::OpenAiCompatibleClient::new(
        "http://127.0.0.1:11434/v1".to_string(),
        None,
    )
    .unwrap();

    let msg = rexos::llm::openai_compat::ChatMessage {
        role: rexos::llm::openai_compat::Role::User,
        content: Some("Reply with the single word: OK".to_string()),
        name: None,
        tool_call_id: None,
        tool_calls: None,
    };

    let res = client
        .chat_completions(rexos::llm::openai_compat::ChatCompletionRequest {
            model,
            messages: vec![msg],
            tools: vec![],
            temperature: Some(0.0),
        })
        .await
        .unwrap();

    let content = res.content.unwrap_or_default();
    assert!(!content.trim().is_empty());
}

