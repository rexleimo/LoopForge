# Routing Recipes (Ollama + Cloud)

Common workflow:

- planning: small/local (cheap, fast)
- coding: stronger cloud model
- summary: cheap summarizer

## Example routing

```toml
[router.planning]
provider = "ollama"
model = "default"

[router.coding]
provider = "glm_native" # or minimax_native / deepseek / kimi / qwen_native ...
model = "default"

[router.summary]
provider = "ollama"
model = "default"
```

See `how-to/providers.md` for full provider examples (GLM/MiniMax native + NVIDIA NIM included).

## Tip: validate with small models first

Validate tool-calling + harness flow with Ollama first, then switch routing to bigger models once the loop is stable.
