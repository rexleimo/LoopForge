# GLM (Zhipu) + MiniMax Native Drivers

**Goal:** Add provider-native (non OpenAI-gateway protocol) drivers for:
- Zhipu GLM (auth token handling + tool calling)
- MiniMax `text/chatcompletion_v2` HTTP API (tool calling)

## Zhipu driver (`zhipu_native`)

- Endpoint: `POST {base_url}/chat/completions`
- Request: OpenAI-style fields plus `stream=false`
- Auth: If `ZHIPUAI_API_KEY` looks like `key_id.key_secret`, generate a short-lived HS256 JWT and send it as `Authorization: Bearer <jwt>`. If it already looks like a JWT (3 segments), pass through.

## MiniMax driver (`minimax_native`)

- Endpoint: `POST {base_url}/text/chatcompletion_v2`
- Request: `model/messages/tools/temperature` + `tool_choice=auto|none` + `stream=false`
- Auth: `Authorization: Bearer <api_key>`

## Tests

Two `axum` mock-server integration tests verify request mapping and tool-call response parsing:
- `tests/llm_zhipu.rs`
- `tests/llm_minimax.rs`

