# Reserved Tools (Phase 1: Agents/Tasks/Events) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement the first reserved tool group in RexOS — `agent_*`, `task_*`, and `event_publish` — so they no longer return “tool not implemented yet”, and so multi-session collaboration flows are possible with persistent state.

**Architecture:** Implement these tools in `rexos-runtime` (like `memory_store/memory_recall`) because they require access to the runtime’s persistent `MemoryStore` and, for `agent_send`, the ability to run nested sessions. Persist state in the existing SQLite KV store using namespaced keys (no schema migration in phase 1). Update tool definitions in `rexos-tools` to provide real parameter schemas and RexOS-native descriptions. Update docs to move implemented tool names out of the “reserved stubs” list.

**Tech Stack:** Rust 2021, `serde_json`, `uuid`, SQLite (rusqlite via `rexos-memory`), axum mock servers for integration tests.

---

### Task 1: Add RED integration tests for `agent_*` tools

**Files:**
- Create: `crates/rexos/tests/reserved_agent_tools.rs`

**Step 1: Write failing tests**
- `agent_spawn` creates an agent record (deterministic `agent_id` supplied) and `agent_list` includes it.
- `agent_find` returns the agent when queried by name.
- `agent_kill` marks the agent as killed and prevents further `agent_send`.

**Step 2: Run test to verify it fails**

Run:

```bash
cargo test -p rexos reserved_agent_tools
```

Expected: FAIL with “tool not implemented yet: agent_*”.

---

### Task 2: Implement `agent_*` tools in the runtime (RED → GREEN)

**Files:**
- Modify: `crates/rexos-runtime/src/lib.rs`

**Implementation notes:**
- Persist agents under KV keys:
  - `rexos.agents.index` → JSON array of agent IDs
  - `rexos.agents.<id>` → JSON record `{id,name,system_prompt,status,created_at,killed_at}`
- `agent_spawn` accepts either:
  - `{ "agent_id"?, "name"?, "system_prompt"? }`, OR
  - `{ "manifest_toml": "..." }` (best-effort parse of `name` + `model.system_prompt`), to support manifest-style spawning.
- `agent_send` runs a nested `run_session()` for the target agent session id and returns the agent’s response text.
- Add a small recursion guard (max depth) for nested agent calls.

**Step 1: Run test to verify it passes**

Run:

```bash
cargo test -p rexos reserved_agent_tools
```

Expected: PASS.

---

### Task 3: Add RED integration tests for `task_*` and `event_publish`

**Files:**
- Create: `crates/rexos/tests/reserved_task_tools.rs`

**Step 1: Write failing tests**
- `task_post` creates a task; `task_list` includes it.
- `task_claim` claims the next pending task (FIFO) for a caller agent id.
- `task_complete` marks as completed and stores a result.
- `event_publish` persists an event in shared memory (KV) and returns an ack string.

**Step 2: Run test to verify it fails**

Run:

```bash
cargo test -p rexos reserved_task_tools
```

Expected: FAIL with “tool not implemented yet: task_* / event_publish”.

---

### Task 4: Implement `task_*` + `event_publish` in the runtime (RED → GREEN)

**Files:**
- Modify: `crates/rexos-runtime/src/lib.rs`

**Implementation notes:**
- Persist tasks under KV keys:
  - `rexos.tasks.index` → JSON array of task IDs (creation order)
  - `rexos.tasks.<id>` → JSON record `{id,title,description,status,assigned_to,claimed_by,result,created_at,claimed_at,completed_at}`
- `task_post` requires `{title, description}` and supports optional `assigned_to`.
- `task_claim` finds the first `pending` task (or pending + assigned_to matching caller) and marks it `claimed`.
- `task_complete` requires `{task_id, result}` and marks task as `completed`.
- `task_list` accepts optional `{status}` filter.
- `event_publish` requires `{event_type}` and optional `{payload}`; append to `rexos.events` KV array (truncate to last N entries).

**Step 1: Run tests**

Run:

```bash
cargo test -p rexos reserved_task_tools
```

Expected: PASS.

---

### Task 5: Update tool definitions and docs

**Files:**
- Modify: `crates/rexos-tools/src/lib.rs`
- Modify: `docs-site/reference/tools.md`
- Modify: `docs-site/zh/reference/tools.md`

**Steps:**
- Replace stub schemas/descriptions for implemented tools with real parameter schemas + RexOS-native descriptions.
- Move implemented tool names out of “Reserved tools (stubs)” sections, leaving the remaining names as stubs.

**Verification:**

Run:

```bash
cargo test
python3 -m mkdocs build --strict
```

Expected: PASS.

---

### Task 6: Commit

Commit message:
- `feat: implement agent/task reserved tools`

