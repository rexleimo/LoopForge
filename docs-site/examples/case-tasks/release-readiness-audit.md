# Release Readiness Audit

**Goal:** generate a release checklist with concrete blockers before tagging.

## Run

1) `cd` into the target repository.

2) Run:

=== "macOS/Linux"
    ```bash
    loopforge agent run --workspace . --prompt "Audit release readiness for this repo. Inspect Cargo.toml (or package metadata), CHANGELOG.md, release workflows in .github/workflows, and scripts/package_release.py (if present). Write notes/release-readiness-audit.md with sections: 1) Version/tag consistency 2) Changelog readiness 3) CI/release workflow readiness 4) Packaging artifact naming checks 5) Blockers (P0/P1) 6) Recommended next release command sequence. Keep it practical and command-oriented."
    ```

=== "Windows (PowerShell)"
    ```powershell
    loopforge agent run --workspace . --prompt "Audit release readiness for this repo. Inspect Cargo.toml (or package metadata), CHANGELOG.md, release workflows in .github/workflows, and scripts/package_release.py (if present). Write notes/release-readiness-audit.md with sections: 1) Version/tag consistency 2) Changelog readiness 3) CI/release workflow readiness 4) Packaging artifact naming checks 5) Blockers (P0/P1) 6) Recommended next release command sequence. Keep it practical and command-oriented."
    ```

## What to expect

- `notes/release-readiness-audit.md`

!!! note
    This task is for preflight analysis. It should not create tags or publish releases. In LoopForge itself, maintainers now typically merge the version/changelog change to `main` and let Actions auto-create the missing semver tag before the existing Release workflow publishes GitHub assets.
