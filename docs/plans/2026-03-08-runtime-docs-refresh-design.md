# Runtime Docs Refresh Design

**Date:** 2026-03-08

## Goal

Refresh the meos documentation so recent runtime refactors are easier to understand without exposing internal competitor analysis or leaking maintainer-only notes into the public docs site.

## Audience split

### Public docs site

Audience:
- advanced users
- integrators
- contributors evaluating LoopForge architecture

Public scope:
- what the major crates/modules do
- how a request flows through runtime, tools, memory, and audits
- where to go next for config, tools, security, and harness docs

Public exclusions:
- competitor references
- internal backlog notes
- “copy from X” style material

### Internal repository docs

Audience:
- maintainers actively refactoring the Rust workspace

Internal scope:
- a concise runtime module map
- current module boundaries after the March refactor waves
- next likely decomposition hotspots

## Recommended approach

1. Add a new public explanation page: `docs-site/explanation/runtime-architecture.md`.
2. Add the matching Chinese page under `docs-site/zh-CN/explanation/runtime-architecture.md`.
3. Add navigation and homepage entry points so users can discover the page.
4. Add README links so GitHub visitors can find architecture docs quickly.
5. Add one internal maintainer doc under `docs/internal/` for the finer-grained runtime map.

## Why this approach

- It keeps user-facing docs focused on product understanding, not internal refactor churn.
- It gives maintainers a separate place to capture code-structure detail.
- It aligns with the existing rule that competitor analysis stays internal only.
