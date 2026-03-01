# GitHub Release Binaries Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Publish prebuilt `rexos` CLI binaries to GitHub Releases so end users can download/run without cloning the repo, and add CI to validate changes with `cargo test`.

**Architecture:** Add two GitHub Actions workflows:
- `CI` workflow runs `cargo test` on PRs/pushes.
- `Release` workflow (tag-triggered) builds `rexos` across a small OS matrix, packages archives, and uploads them to a GitHub Release.

Use a small Python (stdlib-only) packaging script to produce consistent `.tar.gz`/`.zip` archives and `.sha256` checksums across platforms.

**Tech Stack:** GitHub Actions, Rust (cargo), Python 3 (stdlib: `tarfile`, `zipfile`, `hashlib`, `argparse`).

---

### Task 1: Add a cross-platform release packager

**Files:**
- Create: `scripts/package_release.py`

**Step 1: Write a minimal “packager smoke” command (manual test)**

Run (local):
```bash
cargo build --release -p rexos-cli
python3 scripts/package_release.py --version v0.0.0 --target local --bin target/release/rexos --out-dir dist
ls -la dist
```

Expected:
- `dist/rexos-v0.0.0-local.tar.gz`
- `dist/rexos-v0.0.0-local.tar.gz.sha256`

**Step 2: Implement the packaging script**

Requirements:
- Create a staging directory `dist/rexos-<version>-<target>/`
- Copy the built binary into the staging directory (`rexos` or `rexos.exe`)
- Optionally include `README.md` (and `LICENSE` if present)
- Produce:
  - `.zip` for Windows targets
  - `.tar.gz` for everything else
- Write `<archive>.sha256` (SHA-256 hex + two spaces + filename)

**Step 3: Re-run the smoke command**

Expected: archive + checksum created, and the binary is present inside the archive.

**Step 4: Commit**

```bash
git add scripts/package_release.py
git commit -m "build: add release packaging script"
```

---

### Task 2: Add CI workflow (tests)

**Files:**
- Create: `.github/workflows/ci.yml`

**Step 1: Implement CI**

Requirements:
- Trigger on `pull_request` and `push` to `main`
- Run `cargo test` (workspace) on a small OS matrix (Linux/macOS/Windows)
- Use Rust cache (`Swatinem/rust-cache`) for speed

**Step 2: Validate locally**

Run:
```bash
cargo test
```

**Step 3: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add github actions test workflow"
```

---

### Task 3: Add Release workflow (tagged builds → GitHub Releases)

**Files:**
- Create: `.github/workflows/release.yml`

**Step 1: Implement build matrix**

Matrix (initial):
- `ubuntu-latest` → `x86_64-unknown-linux-gnu`
- `macos-13` → `x86_64-apple-darwin`
- `macos-14` → `aarch64-apple-darwin`
- `windows-latest` → `x86_64-pc-windows-msvc`

Each build job:
- `cargo build --release -p rexos-cli --locked`
- `python scripts/package_release.py --version $TAG --target $TARGET ...`
- Upload `dist/*` as workflow artifacts

Release job:
- Download all artifacts into `dist/`
- Create/update GitHub Release for the tag and upload `dist/*` as assets

**Step 2: Document release process**

Update `README.md` with:
- Download-from-Releases instructions for end users
- Maintainer instructions: `git tag vX.Y.Z && git push origin vX.Y.Z` triggers the workflow

**Step 3: Commit**

```bash
git add .github/workflows/release.yml README.md
git commit -m "release: build and upload binaries on tags"
```

---

### Task 4: Final verification

**Step 1: Run tests**

Run:
```bash
cargo test
```

Expected: all tests pass.

**Step 2: Ensure worktree is clean**

Run:
```bash
git status -sb
```

Expected: no uncommitted changes.

