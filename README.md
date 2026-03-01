# RexOS

RexOS is a long-running agent operating system: persistent memory, tool sandboxing, and model routing, plus an Anthropic-style harness for multi-session work.

## Status

This repository is bootstrapped with a long-running harness (`features.json`, `init.sh`, `rexos-progress.md`). Work is tracked by flipping feature `passes` from `false` → `true`.

## Quick start (dev)

```bash
./init.sh
```

## Run with Ollama (OpenAI-compatible)

RexOS defaults to `http://127.0.0.1:11434/v1` in `~/.rexos/config.toml`.

```bash
# 1) Start Ollama
ollama serve

# 2) Init RexOS (creates ~/.rexos/config.toml + ~/.rexos/rexos.db)
cargo run -- init

# 3) Run an agent session in a workspace directory
mkdir -p /tmp/rexos-work
cargo run -- agent run --workspace /tmp/rexos-work --prompt "Create hello.txt with the word hi"
```

To run the optional Ollama smoke test: `REXOS_OLLAMA_MODEL=<your-model> cargo test -- --ignored`.
