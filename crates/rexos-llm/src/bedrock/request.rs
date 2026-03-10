use crate::openai_compat::{ChatMessage, Role, ToolDefinition};
use aws_sdk_bedrockruntime::types::{
    AutoToolChoice, ContentBlock, ConversationRole, Message, SystemContentBlock, Tool, ToolChoice,
    ToolConfiguration, ToolInputSchema, ToolResultBlock, ToolResultContentBlock, ToolResultStatus,
    ToolSpecification, ToolUseBlock,
};

use super::document::json_to_document;

pub(super) fn convert_messages(
    messages: &[ChatMessage],
) -> anyhow::Result<(Vec<SystemContentBlock>, Vec<Message>)> {
    let mut system_blocks = Vec::new();
    let mut bedrock_messages = Vec::new();
    let mut pending_tool_results: Vec<ContentBlock> = Vec::new();

    for message in messages {
        match message.role {
            Role::System => {
                if let Some(text) = message
                    .content
                    .as_ref()
                    .map(|value| value.trim())
                    .filter(|value| !value.is_empty())
                {
                    system_blocks.push(SystemContentBlock::Text(text.to_string()));
                }
            }
            Role::User => {
                flush_tool_results(&mut pending_tool_results, &mut bedrock_messages)?;

                if let Some(text) = message
                    .content
                    .as_ref()
                    .map(|value| value.trim())
                    .filter(|value| !value.is_empty())
                {
                    push_message(
                        &mut bedrock_messages,
                        ConversationRole::User,
                        vec![ContentBlock::Text(text.to_string())],
                    )?;
                }
            }
            Role::Assistant => {
                flush_tool_results(&mut pending_tool_results, &mut bedrock_messages)?;

                let mut blocks = Vec::new();
                if let Some(text) = message
                    .content
                    .as_ref()
                    .map(|value| value.trim())
                    .filter(|value| !value.is_empty())
                {
                    blocks.push(ContentBlock::Text(text.to_string()));
                }
                if let Some(calls) = &message.tool_calls {
                    for call in calls {
                        let input = serde_json::from_str::<serde_json::Value>(
                            call.function.arguments.as_str(),
                        )
                        .unwrap_or(serde_json::Value::Null);
                        let tool_use = ToolUseBlock::builder()
                            .tool_use_id(call.id.clone())
                            .name(call.function.name.clone())
                            .input(json_to_document(&input))
                            .build()
                            .map_err(|err| anyhow::anyhow!("build tool_use: {err}"))?;
                        blocks.push(ContentBlock::ToolUse(tool_use));
                    }
                }

                push_message(&mut bedrock_messages, ConversationRole::Assistant, blocks)?;
            }
            Role::Tool => {
                let tool_use_id = message
                    .tool_call_id
                    .clone()
                    .unwrap_or_else(|| "unknown".into());
                let output = message.content.clone().unwrap_or_default();

                let tool_result = ToolResultBlock::builder()
                    .tool_use_id(tool_use_id)
                    .content(ToolResultContentBlock::Text(output))
                    .status(ToolResultStatus::Success)
                    .build()
                    .map_err(|err| anyhow::anyhow!("build tool_result: {err}"))?;
                pending_tool_results.push(ContentBlock::ToolResult(tool_result));
            }
        }
    }

    flush_tool_results(&mut pending_tool_results, &mut bedrock_messages)?;
    Ok((system_blocks, bedrock_messages))
}

pub(super) fn build_tool_config(
    tools: &[ToolDefinition],
) -> anyhow::Result<Option<ToolConfiguration>> {
    let tools: Vec<Tool> = tools
        .iter()
        .filter(|tool| tool.kind == "function")
        .map(|tool| {
            let input_schema = ToolInputSchema::Json(json_to_document(&tool.function.parameters));
            let spec = ToolSpecification::builder()
                .name(tool.function.name.clone())
                .description(tool.function.description.clone())
                .input_schema(input_schema)
                .build()
                .map_err(|err| anyhow::anyhow!("build tool spec: {err}"))?;
            Ok(Tool::ToolSpec(spec))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    if tools.is_empty() {
        return Ok(None);
    }

    let config = ToolConfiguration::builder()
        .set_tools(Some(tools))
        .tool_choice(ToolChoice::Auto(AutoToolChoice::builder().build()))
        .build()
        .map_err(|err| anyhow::anyhow!("build tool config: {err}"))?;

    Ok(Some(config))
}

fn flush_tool_results(
    pending: &mut Vec<ContentBlock>,
    out: &mut Vec<Message>,
) -> anyhow::Result<()> {
    if pending.is_empty() {
        return Ok(());
    }
    let content = std::mem::take(pending);
    push_message(out, ConversationRole::User, content)?;
    Ok(())
}

fn push_message(
    out: &mut Vec<Message>,
    role: ConversationRole,
    content: Vec<ContentBlock>,
) -> anyhow::Result<()> {
    if content.is_empty() {
        return Ok(());
    }

    let should_merge = out.last().is_some_and(|last| *last.role() == role);
    if should_merge {
        let prev = out
            .pop()
            .ok_or_else(|| anyhow::anyhow!("unexpected empty message list"))?;
        let mut merged = prev.content().to_vec();
        merged.extend(content);
        let merged_msg = Message::builder()
            .role(role)
            .set_content(Some(merged))
            .build()
            .map_err(|err| anyhow::anyhow!("build merged message: {err}"))?;
        out.push(merged_msg);
        return Ok(());
    }

    let msg = Message::builder()
        .role(role)
        .set_content(Some(content))
        .build()
        .map_err(|err| anyhow::anyhow!("build message: {err}"))?;
    out.push(msg);
    Ok(())
}
