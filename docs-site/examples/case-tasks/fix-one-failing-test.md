# Fix One Failing Test (Minimal Change)

**Goal:** identify one failing test, apply the smallest safe fix, and record what changed.

## Run

1) `cd` into the repository.

2) Run:

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "Find the primary test command for this repo (Cargo/npm/pnpm/pytest/go test, etc.), run it once, pick exactly one failing test, and fix only that failure with the smallest safe code change. Then rerun only the relevant tests to confirm the fix. Write notes/fix-one-failing-test.md with: failing symptom, root cause, files changed, verification command, and remaining risks. If no failing tests exist, write notes/fix-one-failing-test.md saying no failure was found and list what command you ran."
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "Find the primary test command for this repo (Cargo/npm/pnpm/pytest/go test, etc.), run it once, pick exactly one failing test, and fix only that failure with the smallest safe code change. Then rerun only the relevant tests to confirm the fix. Write notes/fix-one-failing-test.md with: failing symptom, root cause, files changed, verification command, and remaining risks. If no failing tests exist, write notes/fix-one-failing-test.md saying no failure was found and list what command you ran."
    ```

## What to expect

- `notes/fix-one-failing-test.md`
- One small, focused code/test change (when a real failure exists)

!!! tip "Keep scope tight"
    Add: "Do not refactor unrelated code" to avoid broad edits.
