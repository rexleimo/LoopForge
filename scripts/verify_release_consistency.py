#!/usr/bin/env python3
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


TAG_PATTERN = re.compile(r"^v(\d+)\.(\d+)\.(\d+)$")


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


def parse_release_tag(tag: str) -> str | None:
    m = TAG_PATTERN.fullmatch(tag.strip())
    if not m:
        return None
    return ".".join(m.groups())


def changelog_has_version_section(changelog_text: str, version: str) -> bool:
    pattern = re.compile(rf"^##\s*\[{re.escape(version)}\](?:\s*-.*)?\s*$", re.MULTILINE)
    return pattern.search(changelog_text) is not None


def evaluate_release_consistency(tag: str, cargo_version: str, changelog_text: str) -> tuple[bool, str]:
    tag_version = parse_release_tag(tag)
    if tag_version is None:
        return False, f"release tag '{tag}' must match vX.Y.Z."

    if tag_version != cargo_version:
        return (
            False,
            f"release tag version {tag_version} does not match Cargo.toml workspace version {cargo_version}.",
        )

    if not changelog_has_version_section(changelog_text, tag_version):
        return False, f"CHANGELOG.md is missing section [{tag_version}]."

    return True, f"release metadata is consistent for version {tag_version}."


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Verify release consistency: tag, Cargo.toml workspace version, and CHANGELOG section."
    )
    parser.add_argument("--tag", required=True, help="Release tag, e.g. v0.1.0")
    parser.add_argument("--cargo-toml", default="Cargo.toml", help="Path to Cargo.toml (default: Cargo.toml)")
    parser.add_argument("--changelog", default="CHANGELOG.md", help="Path to CHANGELOG.md (default: CHANGELOG.md)")
    args = parser.parse_args(argv)

    repo_root = Path(__file__).resolve().parents[1]
    cargo_path = (repo_root / args.cargo_toml).resolve()
    changelog_path = (repo_root / args.changelog).resolve()

    try:
        cargo_text = cargo_path.read_text(encoding="utf-8")
        changelog_text = changelog_path.read_text(encoding="utf-8")
    except OSError as err:
        print(f"error: failed to read release metadata files: {err}", file=sys.stderr)
        return 2

    cargo_version = extract_workspace_version(cargo_text)
    if cargo_version is None:
        print("error: failed to parse [workspace.package].version from Cargo.toml.", file=sys.stderr)
        return 2

    ok, message = evaluate_release_consistency(args.tag, cargo_version, changelog_text)
    if ok:
        print(f"ok: {message}")
        return 0
    print(f"error: {message}", file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
