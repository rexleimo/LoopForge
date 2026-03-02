# GUI Smoke Check (example.com)

**Goal:** verify `browser_*` works end-to-end and leaves evidence in your workspace.

See also: [Browser Automation (Playwright)](../browser-automation.md).

## Run

=== "Bash (macOS/Linux)"
    ```bash
    mkdir -p rexos-demo && cd rexos-demo
    export REXOS_BROWSER_HEADLESS=0

    rexos agent run --workspace . --prompt "Use browser tools to open https://example.com, read the page, write a 3-bullet summary to notes/example.md, save a screenshot to .rexos/browser/example.png, then close the browser."
    ```

=== "PowerShell (Windows)"
    ```powershell
    mkdir rexos-demo -Force | Out-Null
    cd rexos-demo
    $env:REXOS_BROWSER_HEADLESS = "0"

    rexos agent run --workspace . --prompt "Use browser tools to open https://example.com, read the page, write a 3-bullet summary to notes/example.md, save a screenshot to .rexos/browser/example.png, then close the browser."
    ```

## What to expect

- `notes/example.md`
- `.rexos/browser/example.png`

## Troubleshooting

- No browser window appears: it’s headless by default; set `REXOS_BROWSER_HEADLESS=0` (or pass `headless=false` to `browser_navigate`).
