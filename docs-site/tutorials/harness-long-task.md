# Long Task With Harness

The harness is for tasks that won’t fit in a single model context window. It makes progress **durable** by combining:

- A workspace directory with durable artifacts (`features.json`, `loopforge-progress.md`, init scripts)
- A verification script (`init.sh` on Unix, `init.ps1` on Windows)
- Git commits as checkpoints
- A session id that persists per-workspace

## 1) Create a workspace

Pick an empty folder for this tutorial:

=== "macOS/Linux"
    ```bash
    mkdir -p loopforge-task
    ```

=== "Windows (PowerShell)"
    ```powershell
    mkdir loopforge-task
    ```

## 2) Initialize the harness

Without a prompt, this only creates the durable artifacts + the initial git commit:

```bash
loopforge harness init loopforge-task
```

With a prompt, LoopForge runs an “initializer agent” to populate `features.json` and adjust the init script:

```bash
loopforge harness init loopforge-task --prompt "Create a small CLI in this workspace that prints Hello and has a passing test suite"
```

## 3) Run an incremental session

```bash
loopforge harness run loopforge-task --prompt "Implement the next failing feature"
```

The harness will:

1. Run `preflight` (show recent commits, progress tail, next failing feature)
2. Run the agent for this session
3. Run the workspace init script
4. If it fails, feed the failure back and retry (up to `--max-attempts`)
5. If it passes, checkpoint commit any changes

## 4) Repeat until done

```bash
loopforge harness run loopforge-task --prompt "Continue"
```

## Where state lives

- Workspace: your code + `features.json` + `loopforge-progress.md` + init scripts + git history
- Memory: `~/.loopforge/loopforge.db` (sessions/messages + small KV store)
