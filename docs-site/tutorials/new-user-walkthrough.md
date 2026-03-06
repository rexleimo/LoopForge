# New User Walkthrough (10 minutes)

This walkthrough is a “sanity check” you can run after installing LoopForge. You’ll verify:

- your local model (Ollama) works
- tools are sandboxed to a workspace
- memory persists across runs
- harness workspaces create durable artifacts + git checkpoints

## 0) Prerequisites

- `loopforge` is installed and on your `PATH`
- Ollama is running: `ollama serve`
- you have at least one **chat model** available:

```bash
ollama list
```

If the default model (`llama3.2`) is not installed, either pull it:

```bash
ollama pull llama3.2
```

…or edit `~/.loopforge/config.toml` and set:

```toml
[providers.ollama]
default_model = "qwen3:4b" # example: pick a model you already have
```

## 0.5) One-command onboarding (recommended)

If you want a single command to run `init + config check + doctor + first task`:

```bash
loopforge onboard --workspace loopforge-onboard-demo
```

Optional:

```bash
# only run setup checks (skip first agent task)
loopforge onboard --workspace loopforge-onboard-demo --skip-agent
```

Expected:
- prints config validation result
- prints doctor summary
- runs one first task in the workspace (unless `--skip-agent`)
- prints `session_id` for continuation

## 1) Initialize LoopForge

```bash
loopforge init
```

Expected artifacts:

- `~/.loopforge/config.toml`
- `~/.loopforge/loopforge.db`

## 2) Run a one-shot agent session (workspace sandbox)

=== "macOS/Linux"
    ```bash
    mkdir -p loopforge-demo
    loopforge agent run --workspace loopforge-demo --prompt "Create hello.txt with the word hi"
    cat loopforge-demo/hello.txt
    ```

=== "Windows (PowerShell)"
    ```powershell
    mkdir loopforge-demo
    loopforge agent run --workspace loopforge-demo --prompt "Create hello.txt with the word hi"
    Get-Content .\loopforge-demo\hello.txt
    ```

Expected:

- `hello.txt` exists in the workspace and contains `hi`
- LoopForge prints a `session_id` to stderr and also persists it under `loopforge-demo/.loopforge/session_id`

## 3) Re-run in the same workspace (memory)

```bash
loopforge agent run --workspace loopforge-demo --prompt "Append a newline + bye to hello.txt"
```

Verify the file updated:

=== "macOS/Linux"
    ```bash
    cat loopforge-demo/hello.txt
    ```

=== "Windows (PowerShell)"
    ```powershell
    Get-Content .\loopforge-demo\hello.txt
    ```

## 4) Create a harness workspace (durable artifacts + git)

=== "macOS/Linux"
    ```bash
    mkdir -p loopforge-harness-demo
    loopforge harness init loopforge-harness-demo
    ```

=== "Windows (PowerShell)"
    ```powershell
    mkdir loopforge-harness-demo
    loopforge harness init loopforge-harness-demo
    ```

Expected files in `loopforge-harness-demo/`:

- `features.json`
- `loopforge-progress.md`
- `init.sh` and `init.ps1`
- a `.git/` directory with an initial commit

Run the harness preflight (no prompt):

```bash
loopforge harness run loopforge-harness-demo
```

## 5) Docs buttons (reproducibility)

On the docs site, every page should have:

- **Edit this page** → opens GitHub at `docs-site/...`
- **View source** → opens the raw Markdown file

If these buttons are missing or broken, check the docs workflow and `mkdocs.yml` (`repo_url` + `edit_uri`).
