use crate::openai_compat::{ChatCompletionRequest, ChatMessage, Role, ToolDefinition};

use super::types::{AnthropicContentBlock, AnthropicMessage, AnthropicRequest, AnthropicTool};

pub(super) fn build_request(req: ChatCompletionRequest) -> anyhow::Result<AnthropicRequest> {
    let (system, messages) = map_messages(&req.messages)?;
    let tools = map_tools(&req.tools);

    Ok(AnthropicRequest {
        model: req.model,
        max_tokens: 1024,
        system,
        messages,
        tools,
    })
}

fn map_tools(tools: &[ToolDefinition]) -> Vec<AnthropicTool> {
    tools
        .iter()
        .filter_map(|tool| {
            if tool.kind != "function" {
                return None;
            }
            Some(AnthropicTool {
                name: tool.function.name.clone(),
                description: tool.function.description.clone(),
                input_schema: tool.function.parameters.clone(),
            })
        })
        .collect()
}

fn map_messages(messages: &[ChatMessage]) -> anyhow::Result<(String, Vec<AnthropicMessage>)> {
    let mut system_parts: Vec<String> = Vec::new();
    let mut out: Vec<AnthropicMessage> = Vec::new();

    for message in messages {
        match message.role {
            Role::System => {
                if let Some(content) = message
                    .content
                    .as_ref()
                    .map(|content| content.trim())
                    .filter(|content| !content.is_empty())
                {
                    system_parts.push(content.to_string());
                }
            }
            Role::User => {
                let mut blocks = Vec::new();
                if let Some(content) = message
                    .content
                    .as_ref()
                    .map(|content| content.trim())
                    .filter(|content| !content.is_empty())
                {
                    blocks.push(AnthropicContentBlock::Text {
                        text: content.to_string(),
                    });
                }
                if !blocks.is_empty() {
                    out.push(AnthropicMessage {
                        role: "user".to_string(),
                        content: blocks,
                    });
                }
            }
            Role::Assistant => {
                let mut blocks = Vec::new();
                if let Some(content) = message
                    .content
                    .as_ref()
                    .map(|content| content.trim())
                    .filter(|content| !content.is_empty())
                {
                    blocks.push(AnthropicContentBlock::Text {
                        text: content.to_string(),
                    });
                }

                if let Some(calls) = &message.tool_calls {
                    for call in calls {
                        let input =
                            serde_json::from_str::<serde_json::Value>(&call.function.arguments)
                                .unwrap_or(serde_json::Value::Null);
                        blocks.push(AnthropicContentBlock::ToolUse {
                            id: call.id.clone(),
                            name: call.function.name.clone(),
                            input,
                        });
                    }
                }

                if !blocks.is_empty() {
                    out.push(AnthropicMessage {
                        role: "assistant".to_string(),
                        content: blocks,
                    });
                }
            }
            Role::Tool => {
                let tool_use_id = message
                    .tool_call_id
                    .as_deref()
                    .ok_or_else(|| anyhow::anyhow!("tool message missing tool_call_id"))?;
                let content = message.content.clone().unwrap_or_default();

                out.push(AnthropicMessage {
                    role: "user".to_string(),
                    content: vec![AnthropicContentBlock::ToolResult {
                        tool_use_id: tool_use_id.to_string(),
                        content,
                        is_error: None,
                    }],
                });
            }
        }
    }

    Ok((system_parts.join("\n\n"), out))
}
