#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

TAG_PATTERN = re.compile(r"^v(\d+)\.(\d+)\.(\d+)$")
VERSION_PATTERN = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")


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


def parse_version(version: str) -> tuple[int, int, int] | None:
    m = VERSION_PATTERN.fullmatch(version.strip())
    if not m:
        return None
    return tuple(int(part) for part in m.groups())


def parse_tag(tag: str) -> tuple[int, int, int] | None:
    m = TAG_PATTERN.fullmatch(tag.strip())
    if not m:
        return None
    return tuple(int(part) for part in m.groups())


def latest_semver_tag(tags: list[str]) -> str | None:
    parsed: list[tuple[tuple[int, int, int], str]] = []
    for tag in tags:
        version = parse_tag(tag)
        if version is None:
            continue
        parsed.append((version, tag.strip()))
    if not parsed:
        return None
    parsed.sort()
    return parsed[-1][1]


def decide_release_tag(
    *,
    workspace_version: str,
    changelog_text: str,
    existing_tags: list[str],
) -> tuple[bool, str | None, str]:
    parsed_workspace = parse_version(workspace_version)
    if parsed_workspace is None:
        raise ValueError(
            f"workspace version '{workspace_version}' must match X.Y.Z before auto release tagging."
        )

    exact_tag = f"v{workspace_version}"
    normalized_tags = [tag.strip() for tag in existing_tags if tag.strip()]
    if exact_tag in normalized_tags:
        return False, None, f"release tag {exact_tag} already exists; nothing to do."

    latest_tag = latest_semver_tag(normalized_tags)
    if latest_tag is not None:
        latest_version = parse_tag(latest_tag)
        assert latest_version is not None
        if parsed_workspace < latest_version:
            raise ValueError(
                f"workspace version {workspace_version} is behind latest release tag {latest_tag}."
            )
        if parsed_workspace == latest_version:
            return (
                False,
                None,
                f"latest release tag version already matches workspace version ({latest_tag}); nothing to do.",
            )

    if not changelog_has_version_section(changelog_text, workspace_version):
        raise ValueError(
            f"CHANGELOG.md is missing section [{workspace_version}] required for auto release tagging."
        )

    return True, exact_tag, f"create release tag {exact_tag} for workspace version {workspace_version}."


def load_git_tags(repo_root: Path) -> list[str]:
    output = subprocess.run(
        ["git", "tag", "--list"],
        cwd=repo_root,
        check=True,
        capture_output=True,
        text=True,
    )
    return [line.strip() for line in output.stdout.splitlines() if line.strip()]


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Resolve whether the current main tip should auto-create a GitHub release tag."
    )
    parser.add_argument("--cargo-toml", default="Cargo.toml")
    parser.add_argument("--changelog", default="CHANGELOG.md")
    args = parser.parse_args(argv)

    repo_root = Path(__file__).resolve().parents[1]
    cargo_path = (repo_root / args.cargo_toml).resolve()
    changelog_path = (repo_root / args.changelog).resolve()

    try:
        cargo_text = cargo_path.read_text(encoding="utf-8")
        changelog_text = changelog_path.read_text(encoding="utf-8")
        existing_tags = load_git_tags(repo_root)
    except subprocess.CalledProcessError as err:
        print(f"error: failed to inspect git tags: {err}", file=sys.stderr)
        return 2
    except OSError as err:
        print(f"error: failed to read release metadata files: {err}", file=sys.stderr)
        return 2

    workspace_version = extract_workspace_version(cargo_text)
    if workspace_version is None:
        print("error: failed to parse [workspace.package].version from Cargo.toml.", file=sys.stderr)
        return 2

    try:
        should_tag, tag, reason = decide_release_tag(
            workspace_version=workspace_version,
            changelog_text=changelog_text,
            existing_tags=existing_tags,
        )
    except ValueError as err:
        print(f"error: {err}", file=sys.stderr)
        return 1

    json.dump(
        {
            "should_tag": should_tag,
            "tag": tag,
            "reason": reason,
            "workspace_version": workspace_version,
            "latest_semver_tag": latest_semver_tag(existing_tags),
        },
        sys.stdout,
    )
    sys.stdout.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
