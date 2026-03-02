# Browser Use Cases (Recipes)

This section is intentionally **copy-paste friendly**: each recipe is a standalone page so you can open it quickly and run it.

See also: [Browser Automation (Playwright)](browser-automation.md).

## Prerequisites (Playwright)

Install Playwright (Python):

```bash
python3 -m pip install playwright
python3 -m playwright install chromium
```

If your Python executable isn't `python3`, set `REXOS_BROWSER_PYTHON` (example: `python`).

## Recipes

- [GUI smoke check (example.com)](browser-use-cases/gui-smoke-check.md)
- [Real-world flow: Baidu “today’s weather” (Browser + Ollama)](browser-use-cases/baidu-weather.md)
- [Wikipedia: open → summarize → screenshot](browser-use-cases/wikipedia-summary.md)
- [(From source) Run the browser + Ollama smoke test](browser-use-cases/smoke-test.md)

## Tips

- For search engines, consider opening a **results URL** directly (more reliable than typing into the homepage search box).
- Always `browser_close` at the end (even on errors).
- Do not enter credentials or complete purchases without explicit user confirmation.
