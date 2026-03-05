# Commit Diff Risk Review

**Goal:** review recent changes and produce a severity-ranked risk list.

## Run

1) `cd` into the repository.

2) Run:

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "Review the most recent 5 commits and changed files in this repository. Write notes/commit-diff-risk-review.md with sections: High risk, Medium risk, Low risk. For each item include why it is risky, likely user impact, and a concrete test or check command. Keep it concise and technical."
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "Review the most recent 5 commits and changed files in this repository. Write notes/commit-diff-risk-review.md with sections: High risk, Medium risk, Low risk. For each item include why it is risky, likely user impact, and a concrete test or check command. Keep it concise and technical."
    ```

## What to expect

- `notes/commit-diff-risk-review.md`

!!! tip
    Add "focus on behavioral regressions" if you want stricter review tone.
