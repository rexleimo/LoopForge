use std::collections::BTreeMap;

use crate::openai_compat::{ChatMessage, Role, ToolDefinition};

use super::types::{
    GeminiContent, GeminiFunctionCall, GeminiFunctionDeclaration, GeminiFunctionResponse,
    GeminiPart, GeminiRequest, GeminiSystemInstruction, GeminiTool,
};

pub(super) fn build_request(
    req: &crate::openai_compat::ChatCompletionRequest,
) -> anyhow::Result<GeminiRequest> {
    let (system_instruction, contents) = map_messages(&req.messages)?;
    let tools = map_tools(&req.tools);

    Ok(GeminiRequest {
        contents,
        system_instruction,
        tools,
    })
}

fn map_tools(tools: &[ToolDefinition]) -> Vec<GeminiTool> {
    let decls: Vec<GeminiFunctionDeclaration> = tools
        .iter()
        .filter_map(|tool| {
            if tool.kind != "function" {
                return None;
            }
            Some(GeminiFunctionDeclaration {
                name: tool.function.name.clone(),
                description: tool.function.description.clone(),
                parameters: tool.function.parameters.clone(),
            })
        })
        .collect();

    if decls.is_empty() {
        Vec::new()
    } else {
        vec![GeminiTool {
            function_declarations: decls,
        }]
    }
}

fn map_messages(
    messages: &[ChatMessage],
) -> anyhow::Result<(Option<GeminiSystemInstruction>, Vec<GeminiContent>)> {
    let mut system_parts = Vec::new();
    let mut out = Vec::new();
    let mut tool_name_by_id: BTreeMap<String, String> = BTreeMap::new();

    for message in messages {
        if let Role::System = message.role {
            if let Some(content) = message
                .content
                .as_ref()
                .map(|content| content.trim())
                .filter(|content| !content.is_empty())
            {
                system_parts.push(content.to_string());
            }
        }

        if let Role::Assistant = message.role {
            if let Some(calls) = &message.tool_calls {
                for call in calls {
                    tool_name_by_id.insert(call.id.clone(), call.function.name.clone());
                }
            }
        }
    }

    for message in messages {
        match message.role {
            Role::System => {}
            Role::User => {
                let text = message
                    .content
                    .as_ref()
                    .map(|content| content.trim())
                    .unwrap_or("");
                if !text.is_empty() {
                    out.push(GeminiContent {
                        role: "user".to_string(),
                        parts: vec![GeminiPart::Text {
                            text: text.to_string(),
                        }],
                    });
                }
            }
            Role::Assistant => {
                let mut parts = Vec::new();
                let text = message
                    .content
                    .as_ref()
                    .map(|content| content.trim())
                    .unwrap_or("");
                if !text.is_empty() {
                    parts.push(GeminiPart::Text {
                        text: text.to_string(),
                    });
                }
                if let Some(calls) = &message.tool_calls {
                    for call in calls {
                        let args =
                            serde_json::from_str::<serde_json::Value>(&call.function.arguments)
                                .unwrap_or(serde_json::Value::Null);
                        parts.push(GeminiPart::FunctionCall {
                            function_call: GeminiFunctionCall {
                                name: call.function.name.clone(),
                                args,
                            },
                        });
                    }
                }
                if !parts.is_empty() {
                    out.push(GeminiContent {
                        role: "model".to_string(),
                        parts,
                    });
                }
            }
            Role::Tool => {
                let tool_use_id = message
                    .tool_call_id
                    .as_deref()
                    .ok_or_else(|| anyhow::anyhow!("tool message missing tool_call_id"))?;
                let name = tool_name_by_id
                    .get(tool_use_id)
                    .ok_or_else(|| anyhow::anyhow!("unknown tool_call_id: {tool_use_id}"))?
                    .clone();
                let output = message.content.clone().unwrap_or_default();
                out.push(GeminiContent {
                    role: "function".to_string(),
                    parts: vec![GeminiPart::FunctionResponse {
                        function_response: GeminiFunctionResponse {
                            name,
                            response: serde_json::json!({ "output": output }),
                        },
                    }],
                });
            }
        }
    }

    let system_instruction = if system_parts.is_empty() {
        None
    } else {
        Some(GeminiSystemInstruction {
            parts: vec![GeminiPart::Text {
                text: system_parts.join("\n\n"),
            }],
        })
    };

    Ok((system_instruction, out))
}
