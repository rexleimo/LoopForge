# RexOS Agent OS Design

**Goal:** Build a long-running agent OS that can execute user instructions over many sessions with persistent memory, sandboxed tools, and model routing — plus an Anthropic-style harness for incremental progress across context windows.

## Architecture (MVP)

RexOS is a CLI-first system with an embedded daemon mode. The core is an **agent runtime** that runs an LLM-driven loop, executes tool calls, persists state to SQLite, and can resume work in later runs. On top of that runtime, RexOS provides a **harness** for long-running tasks: an initializer phase to scaffold a workspace with durable artifacts, followed by repeated incremental coding sessions that always leave the workspace in a clean, resumable state.

Key components:
- **Config**: `~/.rexos/config.toml` defines provider endpoints/keys and routing policy.
- **Memory**: SQLite stores sessions, messages, and a small KV store for agent state.
- **Tools**: capability-scoped tool registry; MVP tools: `fs` (workspace-only), `shell` (workspace-only), and `web_fetch` (basic SSRF protections).
- **LLM drivers**: MVP supports OpenAI-compatible Chat Completions (works with many vendors); later add Anthropic/Gemini native drivers.
- **Model router**: selects models by task class (planning/coding/summarize) and optional budget/limits.
- **Harness**: creates `features.json`, `progress.md`, `init.sh`, and uses git commits as durable checkpoints. Each session: get bearings → run smoke checks → implement one feature → run tests → update `passes` and progress → commit.

## Data flow

1. User launches `rexos harness init <workspace>` or `rexos agent run ...`.
2. RexOS creates/loads a session from SQLite, then renders a system prompt with tool docs, harness rules, and the current workspace state (git log, progress file, failing features).
3. The LLM emits either a final response or tool calls. RexOS executes tools in a sandboxed context and feeds results back.
4. At safe checkpoints, RexOS persists message history + state to SQLite and writes/updates workspace artifacts.
5. The next run resumes from persisted state + durable workspace artifacts.

## Safety / guardrails (MVP)

- Workspace path sandbox: deny path traversal; deny absolute paths unless explicitly allowed.
- Shell tool: runs with a scrubbed environment, fixed working directory, and timeouts.
- Web fetch: deny private IP ranges by default; allowlist option in config.
- Approval hooks: structure in place for future "dangerous action" approvals (network writes, purchases, etc.).

## Testing strategy

- Unit tests for path sandbox, memory persistence, router decisions.
- Integration test for `rexos harness init` creating expected files and git commit.
- Smoke test script `./init.sh` for CI/local verification.

