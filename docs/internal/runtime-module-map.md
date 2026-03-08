# LoopForge Runtime Module Map

This page is internal maintainer documentation.
Do not mirror it into the public docs site.

## Purpose

Capture the current `rexos-runtime` structure after the March readability refactor waves so future cleanup can continue from stable module boundaries instead of rescanning the crate from scratch.

## Top-level families

### `acp`

Owns ACP event persistence and delivery checkpoint storage.

### `agents_hands`

Owns stateful agent-session and Hand lifecycle behavior.
Current split:
- `agents/storage.rs`
- `agents/lifecycle.rs`
- `hands/defs.rs`
- `hands/storage.rs`
- `hands/lifecycle.rs`

### `approval`

Owns approval env parsing and tool-gate policy.
Current split:
- `permissions.rs`
- `tool_gate.rs`

### `knowledge`

Owns runtime-managed knowledge graph storage and query flow.
Current split:
- `storage.rs`
- `mutations.rs`
- `query.rs`

### `outbox`

Owns queued outbound messages and dispatcher helpers.
Current split:
- `store.rs`
- `delivery.rs`

### `scheduling`

Owns schedules and cron-job records.
Current split:
- `storage.rs`
- `schedules.rs`
- `cron.rs`

### `session_runner`

Owns the model/tool loop.
Current split:
- `history.rs`
- `llm.rs`
- `chat_loop.rs`
- `tool_dispatch.rs`
- `tool_processing/events.rs`
- `tool_processing/preflight.rs`
- `tool_processing/outcome.rs`
- `tool_processing/execution.rs`

### `session_skills`

Owns skill allowlists, policy enforcement, and skill audit flow.
Still a good candidate for more decomposition.

### `tasks_events`

Owns task board and event log runtime tools.
Still a good candidate for more decomposition.

### `workflow`

Owns runtime-managed workflow execution and persisted workflow state.
Current split:
- `state.rs`
- `execution.rs`

### `runtime_state` / `runtime_utils`

Hold crate-level shared helpers so `lib.rs` can stay thin.

## Current cleanup principle

Preferred pattern for this crate:

1. keep the top-level module as a thin boundary
2. move storage helpers away from lifecycle code
3. keep policy checks separate from execution logic
4. verify each slice with targeted tests first, then `cargo test --workspace --locked`

## Next bounded hotspots

If continuing the same zero-behavior-change cleanup style, the next good targets are:

1. `crates/rexos-runtime/src/session_runner/tool_dispatch.rs`
2. `crates/rexos-runtime/src/session_skills/audit.rs`
3. `crates/rexos-runtime/src/tasks_events/tasks.rs`

These remain readable enough to work with, but they still bundle multiple concerns compared with the newer module boundaries above.
