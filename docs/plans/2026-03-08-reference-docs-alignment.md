# Reference Docs Alignment Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Refresh the public tools/config reference docs so they better match the current runtime/tool boundaries after the recent refactor waves.

**Architecture:** Keep the reference pages public and user-focused. Add clearer mental models, boundary explanations, and operational notes without exposing maintainer-only or competitor-related content.

**Tech Stack:** MkDocs Material, bilingual Markdown docs, `mkdocs --strict` verification.

---

### Task 1: Strengthen tools reference context

**Files:**
- Modify: `meos/docs-site/reference/tools.md`
- Modify: `meos/docs-site/zh-CN/reference/tools.md`

**Steps:**
1. Add a section explaining standalone tools vs runtime-managed tools.
2. Clarify persistence and dispatch expectations for runtime-managed tools.
3. Document the current `workflow_run` limitation around runtime-managed tools.

### Task 2: Strengthen config reference context

**Files:**
- Modify: `meos/docs-site/reference/config.md`
- Modify: `meos/docs-site/zh-CN/reference/config.md`

**Steps:**
1. Add a top-level config mental model and section map.
2. Add config validation and inspection commands.
3. Add a minimal starter config example and explain how sections work together.

### Task 3: Verify docs

**Files:**
- Verify: `meos/docs-site/reference/*.md`

**Steps:**
1. Run `python3 -m mkdocs build --strict`.
2. Run `cargo test --workspace --locked` as a safety check.
