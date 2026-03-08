# LoopForge (meos) Agent Notes

These instructions apply to the `meos/` repository.

## Public vs internal docs boundary

- **Public docs** are built only from `docs-site/` (see `mkdocs.yml` `docs_dir: docs-site`).
- Keep maintainer maps, strategy notes, and competitor references **out** of `docs-site/`.
- Internal references live under:
  - `docs/internal/` (start here: `docs/internal/index.md`)
  - `docs/plans/`
  - `docs/internal/competitive/`

## Plans and writeups

- Design / decision writeups: `docs/plans/YYYY-MM-DD-<topic>-design.md`
- Implementation plans: `docs/plans/YYYY-MM-DD-<topic>.md`
- Prefer small, focused commits; keep behavior changes separate from refactors/docs when possible.

## Verification (evidence before claims)

Before claiming something “works”, “passes”, “is done”, or before pushing a release tag, run fresh verification:

- `make check`

If you can’t use `make`, run the underlying commands:

- `cargo fmt --all --check`
- `cargo test --workspace --locked`
- `python3 -m mkdocs build --strict` (when docs change)

## Versioning + changelog rule

If an iteration is marked “needs version bump”, the same change set must include:

- `Cargo.toml` `[workspace.package].version`
- matching section in `CHANGELOG.md`

The PR/commit is not releasable if only one of them changes.

## Docs toolchain

Docs deps are pinned in `requirements-docs.txt`. Recommended local setup:

- `make docs-venv` (creates `.venv-docs/` and installs deps)
- `make docs`

## Release preflight (maintainers)

- `loopforge release check --tag vX.Y.Z`
- Avoid publishing competitor content: release check treats this as a blocker.
