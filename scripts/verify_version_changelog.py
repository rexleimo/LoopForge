#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

TRACKED_DOC_VERSION_FILES = (
    "README.md",
    "docs/versioning-and-release.md",
)


def extract_workspace_version(cargo_toml: str) -> str | None:
    in_workspace_package = False
    for raw_line in cargo_toml.splitlines():
        line = raw_line.strip()
        if line.startswith("[") and line.endswith("]"):
            in_workspace_package = line == "[workspace.package]"
            continue
        if not in_workspace_package:
            continue
        m = re.match(r'version\s*=\s*"([^"]+)"', line)
        if m:
            return m.group(1)
    return None


def changelog_has_version_section(changelog_text: str, version: str) -> bool:
    pattern = re.compile(rf"^##\s*\[{re.escape(version)}\](?:\s*-.*)?\s*$", re.MULTILINE)
    return pattern.search(changelog_text) is not None


def contains_version_token(text: str, version: str) -> bool:
    pattern = re.compile(rf"(?<![0-9])v?{re.escape(version)}(?![0-9])")
    return pattern.search(text) is not None


def evaluate_rule(
    *,
    base_version: str,
    head_version: str,
    changed_files: set[str],
    changelog_text: str,
    tracked_docs: dict[str, tuple[str, str]] | None = None,
) -> tuple[bool, str]:
    if base_version == head_version:
        return True, f"workspace version unchanged ({head_version}); rule not triggered."

    if "CHANGELOG.md" not in changed_files:
        return (
            False,
            f"workspace version changed {base_version} -> {head_version}; CHANGELOG.md must be updated in the same change.",
        )

    if not changelog_has_version_section(changelog_text, head_version):
        return (
            False,
            f"workspace version changed {base_version} -> {head_version}; CHANGELOG.md is missing section [{head_version}].",
        )

    if tracked_docs:
        for path, texts in tracked_docs.items():
            base_text, head_text = texts
            if not contains_version_token(base_text, base_version):
                continue

            if path not in changed_files:
                return (
                    False,
                    f"workspace version changed {base_version} -> {head_version}; {path} must be updated in the same change.",
                )

            if not contains_version_token(head_text, head_version):
                return (
                    False,
                    f"workspace version changed {base_version} -> {head_version}; {path} is missing new version {head_version}.",
                )

    return True, f"workspace version changed {base_version} -> {head_version}; version+changelog rule satisfied."


def git_output(repo_root: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=repo_root,
        text=True,
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)} failed: {result.stderr.strip()}")
    return result.stdout


def load_file_at_ref(repo_root: Path, ref: str, rel_path: str) -> str:
    return git_output(repo_root, "show", f"{ref}:{rel_path}")


def changed_files_between(repo_root: Path, base_ref: str, head_ref: str) -> set[str]:
    out = git_output(repo_root, "diff", "--name-only", f"{base_ref}..{head_ref}")
    return {line.strip() for line in out.splitlines() if line.strip()}


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Enforce: when workspace version changes, CHANGELOG.md must be updated with target version section."
    )
    parser.add_argument("--base-ref", required=True, help="Base git ref/sha for comparison.")
    parser.add_argument("--head-ref", default="HEAD", help="Head git ref/sha for comparison (default: HEAD).")
    args = parser.parse_args(argv)

    repo_root = Path(__file__).resolve().parents[1]

    try:
        changed_files = changed_files_between(repo_root, args.base_ref, args.head_ref)
        if "Cargo.toml" not in changed_files:
            print("ok: Cargo.toml unchanged; version/changelog rule not triggered.")
            return 0

        base_cargo = load_file_at_ref(repo_root, args.base_ref, "Cargo.toml")
        head_cargo = load_file_at_ref(repo_root, args.head_ref, "Cargo.toml")
        base_version = extract_workspace_version(base_cargo)
        head_version = extract_workspace_version(head_cargo)
        if base_version is None or head_version is None:
            print(
                "error: failed to parse [workspace.package].version from Cargo.toml in base/head refs.",
                file=sys.stderr,
            )
            return 2

        changelog_text = ""
        if "CHANGELOG.md" in changed_files:
            changelog_text = load_file_at_ref(repo_root, args.head_ref, "CHANGELOG.md")

        tracked_docs: dict[str, tuple[str, str]] = {}
        for rel_path in TRACKED_DOC_VERSION_FILES:
            try:
                base_text = load_file_at_ref(repo_root, args.base_ref, rel_path)
                head_text = load_file_at_ref(repo_root, args.head_ref, rel_path)
            except RuntimeError:
                continue
            tracked_docs[rel_path] = (base_text, head_text)

        ok, message = evaluate_rule(
            base_version=base_version,
            head_version=head_version,
            changed_files=changed_files,
            changelog_text=changelog_text,
            tracked_docs=tracked_docs,
        )
        if ok:
            print(f"ok: {message}")
            return 0
        print(f"error: {message}", file=sys.stderr)
        return 1
    except RuntimeError as err:
        print(f"error: {err}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
