# DashScope (Qwen) Native Driver

**Goal:** Add a provider-native LLM driver for Alibaba DashScope (Qwen) using the Generation API (not OpenAI-compatible), including tool calling support.

**Architecture:** RexOS keeps an internal OpenAI-style chat schema (`ChatCompletionRequest` / `ChatMessage` / `ToolDefinition` / `ToolCall`). The DashScope driver adapts that schema to DashScope’s native request/response envelope:
- Request: `POST {base_url}/services/aigc/text-generation/generation`
- Payload: `{ model, input: { messages }, parameters: { result_format="message", temperature?, tools? } }`
- Response: `{ output: { choices: [ { message, finish_reason? } ] } }`

**Testing:** An `axum` mock-server integration test asserts:
- `Authorization: Bearer <key>` header is set
- request nesting (`input.messages`, `parameters.tools`, `parameters.result_format`) is correct
- tool call responses map back into internal `tool_calls`

