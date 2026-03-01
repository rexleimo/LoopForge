#[tokio::test]
#[ignore]
async fn nvidia_nim_openai_compat_smoke() {
    let api_key = std::env::var("NVIDIA_API_KEY")
        .or_else(|_| std::env::var("REXOS_NVIDIA_API_KEY"))
        .expect("set NVIDIA_API_KEY (or REXOS_NVIDIA_API_KEY) to run this test");

    let base_url = std::env::var("REXOS_NVIDIA_BASE_URL")
        .unwrap_or_else(|_| "https://integrate.api.nvidia.com/v1".to_string());
    let model = std::env::var("REXOS_NVIDIA_MODEL")
        .unwrap_or_else(|_| "meta/llama-3.2-3b-instruct".to_string());

    let client = rexos::llm::openai_compat::OpenAiCompatibleClient::new(base_url, Some(api_key))
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
