# Runtime Architecture

This page is for readers who want to understand how LoopForge is put together beyond the quickstart level.

## The short version

LoopForge is organized as a small Rust workspace with clear boundaries:

- `loopforge-cli` — the user-facing CLI (`loopforge init`, `loopforge onboard`, `loopforge agent run`, `loopforge doctor`)
- `rexos-runtime` — the agent runtime: sessions, runtime-managed tools, workflows, audits, approvals, leak guard, and stateful orchestration
- `rexos-tools` — the standalone tool execution layer for filesystem, shell, browser, web, PDF, media, and process tools
- `rexos-memory` — persistent storage for chat history, KV state, tool-call traces, and runtime records
- `rexos-llm` — provider drivers and model routing helpers
- `rexos-kernel` — shared config, security, routing, and path primitives
- `rexos-daemon` — optional HTTP daemon surface
- `rexos-harness` — long-running workspace bootstrap and checkpoint flow

The goal is simple: keep the system local-first, auditable, and easier to evolve without stuffing everything into one file or one crate.

## How one session flows

A typical `loopforge agent run` request follows this path:

1. **CLI layer** reads config, workspace arguments, and routing choice.
2. **Runtime layer** loads session history and prepares the next model request.
3. **LLM driver** turns the request into a provider-specific API call.
4. If the model requests a tool, **tool processing** checks whitelist rules, approval rules, and leak-guard behavior.
5. The tool is executed either by:
   - a **runtime-managed tool** in `rexos-runtime` (for stateful features like tasks, hands, workflows, schedules, knowledge, outbox), or
   - a **standalone tool** in `rexos-tools` (for shell, files, browser, network, PDF, media, and process operations).
6. Tool outputs, audits, and ACP events are persisted in **memory**.
7. The tool result is sent back into the conversation, and the loop continues until the model returns a final answer.

## Why `rexos-runtime` matters

`rexos-runtime` is the stateful core.
It owns the parts that need durable state or policy decisions, including:

- session lifecycle
- approval checks for risky tools
- leak-guard inspection before replaying tool output
- runtime-managed tools such as agents, hands, tasks, schedules, workflows, knowledge, and channels
- ACP events and delivery checkpoints
- audit persistence for tools and skills

Recent refactors intentionally split this crate into smaller focused modules so contributors can reason about one concern at a time.
That improves review quality and makes behavior-preserving changes safer.

## Why `rexos-tools` stays separate

`rexos-tools` is the reusable tool execution layer.
It focuses on deterministic tool definitions and sandboxed execution, for example:

- filesystem read/write helpers
- shell execution in a workspace
- browser automation through CDP
- web fetch with SSRF and egress policy checks
- PDF extraction
- process and media helpers

Keeping these tools outside the runtime makes them easier to test and reuse, while the runtime remains responsible for orchestration and policy.

## Safety model at a glance

LoopForge uses multiple layers instead of one “magic” guard:

- **workspace sandbox** for filesystem and shell tools
- **egress policy + SSRF checks** for outbound HTTP and browser entrypoints
- **approval gates** for higher-risk tool paths
- **leak guard** before tool output is persisted or replayed into later model turns
- **audit records + ACP events** so operators can inspect what happened later

For a user-facing overview, read [Security & Sandboxing](security.md).

## What changed in the recent refactor waves

The main architectural direction did not change.
What changed is the **shape of the codebase**:

- fewer oversized source files
- more module boundaries by concern
- thinner top-level files that point to focused helpers
- better separation between storage, lifecycle logic, policy checks, and execution flow

That makes the project easier to maintain without changing external behavior.

## If you want to go deeper

Start here next:

- [Concepts](concepts.md)
- [Security & Sandboxing](security.md)
- [Providers & Routing](../how-to/providers.md)
- [Tools Reference](../reference/tools.md)
- [CLI Reference](../reference/cli.md)
- [Long Task With Harness](../tutorials/harness-long-task.md)
