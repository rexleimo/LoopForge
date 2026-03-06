# Browser Use Cases (Recipes)

This section is intentionally **copy-paste friendly**: each recipe is a standalone page so you can open it quickly and run it.

See also: [Browser Automation (CDP)](browser-automation.md).

## Prerequisites (default: CDP)

- Install a Chromium-based browser (Chrome/Chromium/Edge).
- If LoopForge can’t find it, set `LOOPFORGE_BROWSER_CHROME_PATH`.

Optional: use the Playwright backend (legacy) by setting `LOOPFORGE_BROWSER_BACKEND=playwright` and following the install steps in [Browser Automation](browser-automation.md).

## Recipes

- [Real-world flow: Baidu “today’s weather” (Browser + Ollama)](browser-use-cases/baidu-weather.md)
- [Wikipedia: open → summarize → screenshot](browser-use-cases/wikipedia-summary.md)

## Tips

- For search engines, consider opening a **results URL** directly (more reliable than typing into the homepage search box).
- Always `browser_close` at the end (even on errors).
- Do not enter credentials or complete purchases without explicit user confirmation.
