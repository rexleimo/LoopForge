# DevX Guardrails Design (AGENTS.md + Makefile + CI fmt gate)

**Status:** approved
**Date:** 2026-03-08

## Goal

Make a fresh clone easier to work on and harder to accidentally “do the wrong thing” by adding lightweight, repo-local guardrails:

- clear agent/maintainer rules (`AGENTS.md`)
- one-command local verification (`Makefile`)
- a fast CI gate for formatting (`cargo fmt --check`)

## Non-goals

- No behavior changes to runtime/CLI/providers.
- No new release/version bump work.
- No new lints that could create high-noise failures (clippy can be added later if desired).
- No restructuring of public docs site navigation.

## Proposed changes

### 1) `AGENTS.md` in repo root (`meos/`)

Add an agent-facing contract covering:

- public vs internal docs boundary (`docs-site/` only publishes; `docs/internal/` never published)
- required “evidence before claims” verification commands
- where to put design/plan docs (`docs/plans/`)
- versioning rule (Cargo workspace version + `CHANGELOG.md` move together when required)
- common commands / paths (config, docs build, release check)

### 2) `Makefile` in repo root (`meos/`)

Provide stable, discoverable commands for common checks:

- `make fmt-check`
- `make test`
- `make docs` (strict mkdocs build)
- `make check` (fmt + tests + docs)
- `make docs-venv` to install docs dependencies into `.venv-docs/` (which is already gitignored)

### 3) CI: add a formatting gate

Add a `cargo fmt --all --check` job to `.github/workflows/ci.yml` so formatting issues fail early and consistently.

## Verification (local)

- `make check`

## Rollout

Land as a small, DX-only change set on `main`.
