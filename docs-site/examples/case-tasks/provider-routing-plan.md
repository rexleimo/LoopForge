# Provider Routing Plan (Local / Team / CI)

**Goal:** draft a practical provider-routing plan for different operating modes.

## Run

1) `cd` into the repository.

2) Run:

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "Read docs that describe providers and routing (for example docs-site/how-to/providers.md and docs-site/reference/config.md). Write notes/provider-routing-plan.md with 3 profiles: Local dev (cheap), Team default (balanced), CI/release checks (stable). For each profile provide provider/model choices, fallback strategy, and expected tradeoffs (cost/latency/quality)."
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "Read docs that describe providers and routing (for example docs-site/how-to/providers.md and docs-site/reference/config.md). Write notes/provider-routing-plan.md with 3 profiles: Local dev (cheap), Team default (balanced), CI/release checks (stable). For each profile provide provider/model choices, fallback strategy, and expected tradeoffs (cost/latency/quality)."
    ```

## What to expect

- `notes/provider-routing-plan.md`

!!! note
    This task drafts a plan first. Apply config changes only after review.
