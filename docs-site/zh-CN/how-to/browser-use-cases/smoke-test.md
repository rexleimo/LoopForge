# Smoke test（从源码）

如果你在开发 RexOS 本身，可以运行这个被 `#[ignore]` 的 smoke test：

```bash
REXOS_OLLAMA_MODEL=qwen3:4b cargo test -p rexos --test browser_baidu_weather_smoke -- --ignored --nocapture
```

预期输出会包含类似：

- `[rexos][baidu_weather] summary=...`

注意：

- 默认浏览器后端是 **CDP**，因此你需要本机安装 Chromium 系浏览器（Chrome/Chromium/Edge）。
- 该测试默认使用临时 workspace 并会自动清理；如果你想保留截图/页面 dump：
  - `export REXOS_BROWSER_SMOKE_WORKSPACE=./rexos-browser-smoke`（或任意目录）

当你设置了 `REXOS_BROWSER_SMOKE_WORKSPACE` 后，该测试会写入：

- `.rexos/browser/baidu_weather.png`
- `notes/baidu_weather_page.txt`
- `notes/weather.md`
