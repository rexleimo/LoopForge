# Concepts

LoopForge is built for workflows that are **not** “one prompt and done”.

## Workspace

A workspace is the working directory for one run.

- filesystem tools and shell commands are constrained to this directory
- harness artifacts are also written here
- browser screenshots, notes, and generated files usually land here as well

## Memory (SQLite)

LoopForge remembers:

- earlier sessions
- chat history
- small pieces of persistent state
- runtime-managed records such as tasks, schedules, workflows, and outbox messages

This state is stored in `~/.loopforge/loopforge.db` so later runs can continue from earlier work.

## Tools (inside guardrails)

An agent can use tools such as:

- `fs_read` / `fs_write` — read and write files inside the workspace
- `shell` — run commands inside the workspace
- `web_fetch` — fetch web pages with SSRF protections by default
- `browser_*` — automate a browser via CDP

!!! note "Browser prerequisites"
    By default LoopForge uses a local Chrome/Chromium/Edge browser through CDP.

    If LoopForge cannot find a browser binary, set `LOOPFORGE_BROWSER_CHROME_PATH`.

    The older fallback path is Playwright bridge mode:

    ```bash
    export LOOPFORGE_BROWSER_BACKEND=playwright
    python3 -m pip install playwright
    python3 -m playwright install chromium
    ```

## Model routing

LoopForge separates work into task kinds:

- `planning`
- `coding`
- `summary`

Each kind can route to a different `(provider, model)` pair.
This makes it possible to use one model for planning, another for code work, and a cheaper one for summaries.

## Harness (durable long tasks)

Harness sits on top of the runtime for longer, checkpointed work:

1. initialize a workspace with starter artifacts
2. run incremental sessions instead of one giant prompt
3. verify with `init.sh` / `init.ps1`
4. checkpoint progress in Git

## Runtime-managed tools

Some LoopForge capabilities are not just one-off tools. They are runtime-managed and keep state across sessions, for example:

- agents and hands
- tasks and events
- schedules and cron jobs
- workflows
- channels and outbox delivery
- knowledge graph records

If you want the full system view, continue to [Runtime Architecture](runtime-architecture.md).
