# Smoke Test (From Source)

If you're hacking on RexOS itself, you can run the ignored smoke test:

```bash
REXOS_OLLAMA_MODEL=qwen3:4b cargo test -p rexos --test browser_baidu_weather_smoke -- --ignored --nocapture
```

Notes:

- The default browser backend is **CDP**, so you need a local Chromium-based browser (Chrome/Chromium/Edge).
- If you want to force the legacy Playwright backend for this smoke test:
  - `export REXOS_BROWSER_BACKEND=playwright`
  - then install Playwright as described in [Browser Automation](../browser-automation.md).
- By default this test uses a temp workspace and cleans it up. If you want to keep screenshots + page dumps:
  - `export REXOS_BROWSER_SMOKE_WORKSPACE=./rexos-browser-smoke` (or any directory)

Expected output includes a line like:

- `[rexos][baidu_weather] summary=...`

When `REXOS_BROWSER_SMOKE_WORKSPACE` is set, the test writes:

- `.rexos/browser/baidu_weather.png`
- `notes/baidu_weather_page.txt`
- `notes/weather.md`
