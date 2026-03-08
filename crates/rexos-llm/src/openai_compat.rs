mod client;
mod mapping;
mod retry;
mod types;

pub use client::OpenAiCompatibleClient;
pub use types::{
    ChatCompletionRequest, ChatCompletionResponse, ChatMessage, Choice, Role, ToolCall,
    ToolDefinition, ToolFunction, ToolFunctionDefinition,
};
