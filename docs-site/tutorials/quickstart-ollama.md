# Quickstart (Ollama)

## What success looks like

By the end of this tutorial, Ollama should be reachable, `~/.loopforge/config.toml` should be valid, and one workspace artifact should be created by `loopforge agent run`.

This tutorial runs LoopForge locally using Ollama’s OpenAI-compatible endpoint.

## Prerequisites

- Ollama is installed and running.
- You have at least one **chat model** available (example: `qwen3:4b`, `llama3.2`, etc.). (Embedding-only models won’t work.)

Check your local models:

```bash
ollama list
```

LoopForge defaults to `providers.ollama.default_model = "llama3.2"` in `~/.loopforge/config.toml`.

If you don’t have `llama3.2` installed, pick one of these:

1) Pull it:

```bash
ollama pull llama3.2
```

2) Or switch LoopForge to a model you already have (example: `qwen3:4b`):

```toml
[providers.ollama]
default_model = "qwen3:4b"
```

## 1) Start Ollama

```bash
ollama serve
```

## 2) Initialize LoopForge

This creates:
- `~/.loopforge/config.toml` (provider config + routing)
- `~/.loopforge/loopforge.db` (SQLite memory)

```bash
loopforge init
```

## 3) Run your first agent session

Pick a workspace directory (tools are sandboxed to this root):

=== "macOS/Linux"
    ```bash
    mkdir -p loopforge-work
    loopforge agent run --workspace loopforge-work --prompt "Create hello.txt with the word hi"
    cat loopforge-work/hello.txt
    ```

=== "Windows (PowerShell)"
    ```powershell
    mkdir loopforge-work
    loopforge agent run --workspace loopforge-work --prompt "Create hello.txt with the word hi"
    Get-Content .\loopforge-work\hello.txt
    ```

LoopForge prints the final assistant output, and persists a stable `session_id` under `loopforge-work/.loopforge/session_id`.

## 4) Re-run in the same workspace (optional)

```bash
loopforge agent run --workspace loopforge-work --prompt "Now append a newline + bye to hello.txt"
```

## Next steps

- Use the harness for long tasks: see “Long Task With Harness”.
- Switch providers/models: see “Providers & Routing”.
