# Multi-Provider LLM Support Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add multi-provider LLM drivers (OpenAI-compatible, Anthropic, Gemini) with unified tool calling, plus presets for popular providers (MinMax, GLM-5, Kimi, Qwen, DeepSeek).

**Architecture:** Introduce a `LlmDriver` abstraction that accepts/returns LoopForge’s internal chat types (`ChatMessage`, `ToolDefinition`, `ToolCall`). Providers register as named configs (e.g. `ollama`, `anthropic`, `gemini`, `deepseek`), and the router selects `provider + model` per `TaskKind`. OpenAI-compatible providers share one driver implementation. Anthropic and Gemini use native APIs with adapters to/from internal tool-calling structures.

**Tech Stack:** Rust 2021, `reqwest`, `tokio`, `serde`, `toml`, `axum` (tests).

---

### Task 1: Refactor config for multi-provider

**Files:**
- Modify: `src/config.rs`
- Modify: `src/router/mod.rs`
- Test: `src/config.rs` (unit tests)

**Step 1: Write failing config test**

- Add a test that expects:
  - `providers.<name>.kind/base_url/api_key_env/default_model` serialize
  - `router.<kind>.provider/router.<kind>.model` serialize

**Step 2: Run tests to verify RED**

Run: `cargo test config::tests::...`
Expected: FAIL due to missing config structs/fields.

**Step 3: Implement config structs + defaults**

- Add:
  - `ProviderKind` enum: `openai_compatible | anthropic | gemini`
  - `ProviderConfig` struct
  - `providers: BTreeMap<String, ProviderConfig>`
  - `router: RouterConfig` where each `RouteConfig` has `provider` and `model`
- Keep a sensible default that works with Ollama on `http://127.0.0.1:11434/v1`.

**Step 4: Run tests to verify GREEN**

Run: `cargo test`
Expected: PASS.

**Step 5: Commit**

Run:
```bash
git add src/config.rs src/router/mod.rs
git commit -m "feat: multi-provider config + routing"
```

---

### Task 2: Add `LlmDriver` trait + provider registry

**Files:**
- Create: `src/llm/driver.rs`
- Create: `src/llm/registry.rs`
- Modify: `src/llm/mod.rs`
- Test: `src/llm/registry.rs` (unit tests)

**Step 1: Write failing registry test**

- Given config with multiple providers, expect:
  - registry can build drivers
  - lookup by name returns a driver

**Step 2: Run tests (RED)**

Run: `cargo test llm::registry::tests::...`
Expected: FAIL.

**Step 3: Implement minimal registry**

- `trait LlmDriver { async fn chat(&self, req: ChatCompletionRequest) -> Result<ChatMessage>; }`
- `LlmRegistry::from_config(...)` builds:
  - OpenAI-compatible driver(s)
  - Anthropic driver(s)
  - Gemini driver(s)

**Step 4: Run tests (GREEN)**

Run: `cargo test`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/llm
git commit -m "feat: llm driver trait and registry"
```

---

### Task 3: Implement Anthropic driver + tool adapter

**Files:**
- Create: `src/llm/anthropic.rs`
- Modify: `src/llm/mod.rs`
- Test: `tests/llm_anthropic.rs`

**Step 1: Write failing test (mock server)**

- Use `axum` to stand up a local server.
- Assert LoopForge sends:
  - `system` string (from internal system message)
  - `messages` list (user/assistant)
  - `tools` in Anthropic schema
- Return a response containing a `tool_use` block and verify:
  - driver returns `ChatMessage { role=Assistant, tool_calls=[...] }`

**Step 2: Run test (RED)**

Run: `cargo test --test llm_anthropic`
Expected: FAIL.

**Step 3: Implement minimal Anthropic driver**

- Request: `POST {base_url}/v1/messages`
- Headers: `x-api-key`, `anthropic-version`, `content-type: application/json`
- Response parsing:
  - collect text blocks into `content`
  - collect tool blocks into internal `tool_calls`

**Step 4: Run test (GREEN)**

Run: `cargo test --test llm_anthropic`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/llm/anthropic.rs tests/llm_anthropic.rs
git commit -m "feat: anthropic driver with tool calling"
```

---

### Task 4: Implement Gemini driver + tool adapter

**Files:**
- Create: `src/llm/gemini.rs`
- Modify: `src/llm/mod.rs`
- Test: `tests/llm_gemini.rs`

**Step 1: Write failing test (mock server)**

- Verify request uses API key and correct endpoint.
- Return `functionCall` result and verify it maps to internal `tool_calls`.

**Step 2: Run test (RED)**

Run: `cargo test --test llm_gemini`
Expected: FAIL.

**Step 3: Implement minimal Gemini driver**

- Request: `POST {base_url}/models/{model}:generateContent?key=...` (or configurable)
- Adapter:
  - internal tools → Gemini function declarations
  - Gemini function call → internal tool call

**Step 4: Run test (GREEN)**

Run: `cargo test --test llm_gemini`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/llm/gemini.rs tests/llm_gemini.rs
git commit -m "feat: gemini driver with tool calling"
```

---

### Task 5: Wire agent runtime + CLI to provider routing

**Files:**
- Modify: `src/agent/mod.rs`
- Modify: `src/main.rs`
- Test: `tests/agent_loop.rs` (extend)

**Step 1: Write failing test**

- Configure router to pick a named provider.
- Ensure the chosen provider’s base_url is used (mock server asserts).

**Step 2: Run test (RED)**

Run: `cargo test --test agent_loop`
Expected: FAIL.

**Step 3: Implement wiring**

- `AgentRuntime` uses `LlmRegistry` + router to select driver per run.
- OpenAI-compatible remains default for Ollama.

**Step 4: Run tests (GREEN)**

Run: `cargo test`
Expected: PASS.

**Step 5: Commit**

```bash
git add src/agent/mod.rs src/main.rs tests/agent_loop.rs
git commit -m "feat: route provider+model for agent sessions"
```

---

### Task 6: Add presets for popular providers (OpenAI-compatible)

**Files:**
- Modify: `src/config.rs`
- Modify: `README.md`
- Test: `src/config.rs` (default serialization)

**Step 1: Add provider entries**

Add reasonable default stubs (base URLs + env vars) for:
- `deepseek`
- `kimi` (Moonshot)
- `qwen` (Alibaba)
- `glm` (Zhipu / GLM-5)
- `minmax`

**Step 2: Document how to enable**

- Show config snippets and required env vars.

**Step 3: Commit**

```bash
git add src/config.rs README.md
git commit -m "docs: provider presets (deepseek/kimi/qwen/glm/minmax)"
```

