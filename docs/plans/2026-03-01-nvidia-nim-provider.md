# NVIDIA NIM Provider Preset Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add an out-of-the-box NVIDIA NIM provider preset (OpenAI-compatible) and document how to use it.

**Architecture:** Treat NVIDIA NIM as `openai_compatible` (Chat Completions). Add a `providers.nvidia` preset in the default `~/.loopforge/config.toml` generation, plus docs examples and env var setup.

**Tech Stack:** Rust (Cargo workspace), `reqwest` (OpenAI-compatible client), MkDocs Material + `mkdocs-static-i18n` docs site.

---

### Task 1: Add a failing test for the NVIDIA preset

**Files:**
- Modify: `meos/crates/rexos-kernel/src/config.rs`
- Test: `meos/crates/rexos-kernel/src/config.rs`

**Step 1: Write the failing test**

Add one assertion to `default_config_serializes()`:

```rust
assert!(toml_str.contains("[providers.nvidia]"));
```

**Step 2: Run test to verify it fails**

Run:

```bash
cargo test -p rexos-kernel default_config_serializes
```

Expected: FAIL because the default config doesn’t include `providers.nvidia` yet.

---

### Task 2: Add a failing preset sanity test (base URL)

**Files:**
- Modify: `meos/crates/rexos-kernel/src/config.rs`
- Test: `meos/crates/rexos-kernel/src/config.rs`

**Step 1: Write the failing test**

Add:

```rust
#[test]
fn nvidia_preset_uses_nim_base_url() {
    let cfg = RexosConfig::default();
    let nvidia = cfg.providers.get("nvidia").unwrap();
    assert_eq!(nvidia.base_url, "https://integrate.api.nvidia.com/v1");
}
```

**Step 2: Run test to verify it fails**

Run:

```bash
cargo test -p rexos-kernel nvidia_preset_uses_nim_base_url
```

Expected: FAIL until the `nvidia` preset exists.

---

### Task 3: Implement the NVIDIA preset in default config

**Files:**
- Modify: `meos/crates/rexos-kernel/src/config.rs`
- Test: `meos/crates/rexos-kernel/src/config.rs`

**Step 1: Write minimal implementation**

In `RexosConfig::default()`, insert:

```rust
providers.insert(
    "nvidia".to_string(),
    ProviderConfig {
        kind: ProviderKind::OpenAiCompatible,
        base_url: "https://integrate.api.nvidia.com/v1".to_string(),
        api_key_env: "NVIDIA_API_KEY".to_string(),
        default_model: "meta/llama-3.2-3b-instruct".to_string(),
    },
);
```

**Step 2: Run tests to verify they pass**

Run:

```bash
cargo test -p rexos-kernel default_config_serializes
cargo test -p rexos-kernel nvidia_preset_uses_nim_base_url
```

Expected: PASS.

---

### Task 4: Update docs (English + Chinese)

**Files:**
- Modify: `meos/docs-site/how-to/providers.md`
- Modify: `meos/docs-site/zh/how-to/providers.md`
- Modify: `meos/docs-site/index.md`
- Modify: `meos/docs-site/zh/index.md`
- Modify: `meos/README.md`
- Modify: `meos/README.zh-CN.md`

**Step 1: Add preset mention**

Add `nvidia` to the built-in presets list.

**Step 2: Add a provider example**

Add a new section “Example: NVIDIA NIM” with:
- `base_url = "https://integrate.api.nvidia.com/v1"`
- `api_key_env = "NVIDIA_API_KEY"`
- `default_model = "meta/llama-3.2-3b-instruct"`

**Step 3: Add env var instructions**

Add `NVIDIA_API_KEY` to the API key lists (Bash + PowerShell).

**Step 4: Validate docs build**

Run:

```bash
python3 -m mkdocs build --strict
```

Expected: success.
