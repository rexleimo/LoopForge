# Regression Test Gap Map

**Goal:** find risky areas that lack regression tests and propose targeted test cases.

## Run

1) `cd` into the repository.

2) Run:

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "Inspect src/ (or equivalent code folders), existing test folders, and recent changes. Write notes/regression-test-gap.md with: 1) critical behaviors that could regress 2) current test coverage signals 3) top 5 missing regression tests (with test names and what each asserts) 4) the first test you would implement now and why."
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "Inspect src/ (or equivalent code folders), existing test folders, and recent changes. Write notes/regression-test-gap.md with: 1) critical behaviors that could regress 2) current test coverage signals 3) top 5 missing regression tests (with test names and what each asserts) 4) the first test you would implement now and why."
    ```

## What to expect

- `notes/regression-test-gap.md`

!!! tip
    Ask for "prioritize user-facing failures" to keep the list actionable.
