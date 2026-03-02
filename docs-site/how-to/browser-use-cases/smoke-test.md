# Smoke Test (From Source)

If you're hacking on RexOS itself, you can run the ignored smoke test:

```bash
REXOS_OLLAMA_MODEL=qwen3:4b cargo test -p rexos --test browser_baidu_weather_smoke -- --ignored --nocapture
```

Expected output includes a line like:

- `[rexos][baidu_weather] summary=...`

This test uses a temp workspace and cleans it up. Use the other recipes if you want to keep screenshots and files.
