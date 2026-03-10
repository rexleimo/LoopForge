#[cfg(feature = "bedrock")]
use rexos::llm::driver::LlmDriver;

#[cfg(feature = "bedrock")]
#[tokio::test]
#[ignore]
async fn bedrock_smoke() {
    let model = std::env::var("LOOPFORGE_BEDROCK_MODEL")
        .expect("set LOOPFORGE_BEDROCK_MODEL to run this test");
    let region =
        std::env::var("LOOPFORGE_BEDROCK_REGION").unwrap_or_else(|_| "us-east-1".to_string());
    let cross_region = std::env::var("LOOPFORGE_BEDROCK_CROSS_REGION").unwrap_or_default();
    let profile = std::env::var("LOOPFORGE_BEDROCK_PROFILE").unwrap_or_default();

    let aws_cfg = rexos::config::AwsBedrockConfig {
        region,
        cross_region,
        profile,
    };

    let driver = rexos::llm::bedrock::BedrockDriver::new(Some(&aws_cfg)).unwrap();

    let msg = driver
        .chat(rexos::llm::openai_compat::ChatCompletionRequest {
            model,
            messages: vec![rexos::llm::openai_compat::ChatMessage {
                role: rexos::llm::openai_compat::Role::User,
                content: Some("Reply with the single word: OK".to_string()),
                name: None,
                tool_call_id: None,
                tool_calls: None,
            }],
            tools: vec![],
            temperature: Some(0.0),
        })
        .await
        .unwrap();

    let content = msg.content.unwrap_or_default();
    assert!(!content.trim().is_empty());
}
