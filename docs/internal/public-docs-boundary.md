# Public Docs Boundary

## Rule

Only content under `docs-site/` and entries wired into `mkdocs.yml` may appear on the public LoopForge docs site.

Competitor analysis, positioning teardown, borrowing notes, and internal strategy material must stay under internal-only repository paths such as:

- `docs/internal/`
- `docs/plans/`
- other non-MkDocs internal folders

## Competitor Content Policy

The following categories are internal-only and must not be published in the public docs site:

- direct competitor comparisons
- "what we borrow from competitor X" writeups
- internal differentiation strategy notes
- positioning language that frames LoopForge against named competitors

Current internal archive location:

- `docs/internal/competitive/`

## Enforcement

Run `loopforge release check --tag vX.Y.Z` before any release tag work.
The release check now fails if public docs inputs (`docs-site/` or `mkdocs.yml`) contain competitor-analysis terms such as `OpenFang` or `OpenClaw`.

