# Runtime Docs Refresh Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Update meos documentation so the refactored runtime structure is easier to understand for both public readers and internal maintainers.

**Architecture:** Add one public runtime architecture page plus one internal maintainer map, then wire both through existing navigation and README entry points. Keep competitor-related material strictly out of public docs.

**Tech Stack:** MkDocs Material, i18n folder-based docs, Markdown, `mkdocs --strict` verification.

---

### Task 1: Add public runtime architecture docs

**Files:**
- Create: `meos/docs-site/explanation/runtime-architecture.md`
- Create: `meos/docs-site/zh-CN/explanation/runtime-architecture.md`
- Modify: `meos/mkdocs.yml`

**Steps:**
1. Add an English runtime architecture page.
2. Add a Simplified Chinese runtime architecture page.
3. Add both pages to the Explanation nav and translation labels.

### Task 2: Add maintainer-facing internal map

**Files:**
- Create: `meos/docs/internal/runtime-module-map.md`

**Steps:**
1. Summarize current runtime module boundaries.
2. Capture the purpose of each module family.
3. Note next bounded hotspots for future cleanup.

### Task 3: Add discovery links

**Files:**
- Modify: `meos/docs-site/index.md`
- Modify: `meos/docs-site/zh-CN/index.md`
- Modify: `meos/README.md`
- Modify: `meos/README.zh-CN.md`

**Steps:**
1. Add homepage links/cards for runtime architecture.
2. Add README docs links for architecture and internal maintainer map.
3. Keep wording audience-aware and avoid internal-only content on the public site.

### Task 4: Verify docs

**Files:**
- Verify: `meos/docs-site/**/*.md`
- Verify: `meos/mkdocs.yml`

**Steps:**
1. Run `python3 -m mkdocs build --strict`.
2. Run `cargo test --workspace --locked` as a safety check because README paths and docs references changed.
