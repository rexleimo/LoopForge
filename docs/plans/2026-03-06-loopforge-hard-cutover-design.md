# LoopForge Hard Cutover Design

**Goal:** Remove outward-facing `LoopForge` branding and compatibility surfaces so users only see and use `LoopForge`.

## Scope
- Change public/product copy from `LoopForge` to `LoopForge`.
- Hard-cut runtime-facing names from `rexos` to `loopforge` where users interact with them directly.
- Remove compatibility wording such as ``, ``, and legacy `.loopforge` / `LOOPFORGE_*` guidance.
- Keep internal crate/package names if they are not part of user-facing UX.

## User-Facing Surfaces To Cut Over
- CLI help/about and command docs.
- Config/data directory examples and defaults: `~/.loopforge`.
- Workspace artifact directory examples and defaults: `.loopforge/...`.
- Harness artifact names shown to users: `loopforge-progress.md`.
- Public environment variable names: `LOOPFORGE_*`.
- Docs, README, MkDocs repo links, blog copy, and setup scripts.

## Non-Goals
- Renaming Rust crate directories such as `crates/rexos-*`.
- Renaming internal package names unless required by user-visible output.
- Preserving fallback support for `.loopforge`, `LOOPFORGE_*`, or legacy skill lookup.

## Risk Controls
- Add/adjust tests before code changes for path defaults, CLI branding, skill discovery, harness artifacts, and provider script env names.
- Verify with targeted tests first, then docs build, then broader Rust/script checks.
