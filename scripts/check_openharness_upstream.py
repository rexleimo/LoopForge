#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

DEFAULT_REPO_URL = "https://github.com/MaxGfeller/open-harness.git"
DEFAULT_STATE_FILE = "docs/internal/competitive/open-harness-upstream.json"
SHA1_HEX_PATTERN = re.compile(r"^[0-9a-f]{40}$")


def utc_now_iso() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def resolve_path(repo_root: Path, raw: str) -> Path:
    path = Path(raw)
    if path.is_absolute():
        return path
    return repo_root / path


def parse_ls_remote_head(output: str) -> str:
    for raw_line in output.splitlines():
        line = raw_line.strip()
        if not line:
            continue
        parts = line.split()
        if len(parts) >= 2 and parts[1] == "HEAD" and SHA1_HEX_PATTERN.fullmatch(parts[0]):
            return parts[0]
    raise ValueError("failed to parse `git ls-remote <repo> HEAD` output")


def fetch_remote_head(repo_url: str, repo_root: Path) -> str:
    out = subprocess.run(
        ["git", "ls-remote", repo_url, "HEAD"],
        cwd=repo_root,
        check=True,
        capture_output=True,
        text=True,
    )
    return parse_ls_remote_head(out.stdout)


def load_state(path: Path) -> dict:
    if not path.is_file():
        return {}
    return json.loads(path.read_text(encoding="utf-8"))


def save_state(path: Path, state: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(state, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def build_report(
    *,
    repo_url: str,
    previous_head: str | None,
    current_head: str,
    checked_at: str,
) -> dict:
    changed = previous_head is not None and previous_head != current_head
    report: dict[str, object] = {
        "tracked_repo": repo_url,
        "checked_at": checked_at,
        "previous_known_head": previous_head,
        "current_head": current_head,
        "has_baseline": previous_head is not None,
        "changed": changed,
        "head_url": f"https://github.com/MaxGfeller/open-harness/commit/{current_head}",
    }
    if changed:
        report["compare_url"] = (
            f"https://github.com/MaxGfeller/open-harness/compare/{previous_head}...{current_head}"
        )
    return report


def render_markdown(report: dict) -> str:
    changed = bool(report.get("changed"))
    status = "changed" if changed else "stable"
    rows = [
        "| field | value |",
        "| --- | --- |",
        f"| tracked_repo | `{report.get('tracked_repo')}` |",
        f"| checked_at | `{report.get('checked_at')}` |",
        f"| previous_known_head | `{report.get('previous_known_head')}` |",
        f"| current_head | `{report.get('current_head')}` |",
        f"| status | `{status}` |",
    ]
    compare_url = report.get("compare_url")
    if compare_url:
        rows.append(f"| compare_url | {compare_url} |")
    head_url = report.get("head_url")
    if head_url:
        rows.append(f"| head_url | {head_url} |")
    return "# Open-Harness Upstream Watch Report\n\n" + "\n".join(rows) + "\n"


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Track upstream HEAD for MaxGfeller/open-harness and report drift from local baseline."
    )
    parser.add_argument("--repo-url", default=DEFAULT_REPO_URL)
    parser.add_argument("--state-file", default=DEFAULT_STATE_FILE)
    parser.add_argument("--out-json", default="")
    parser.add_argument("--out-markdown", default="")
    parser.add_argument("--update-state", action="store_true")
    parser.add_argument("--fail-on-change", action="store_true")
    parser.add_argument("--checked-at", default="")
    args = parser.parse_args(argv)

    repo_root = Path(__file__).resolve().parents[1]
    state_path = resolve_path(repo_root, args.state_file)

    try:
        state = load_state(state_path)
        previous_head = state.get("last_known_head")
        current_head = fetch_remote_head(args.repo_url, repo_root)
    except subprocess.CalledProcessError as err:
        print(f"error: failed to query upstream head: {err}", file=sys.stderr)
        return 2
    except (OSError, ValueError, json.JSONDecodeError) as err:
        print(f"error: failed to load/parse upstream state: {err}", file=sys.stderr)
        return 2

    checked_at = args.checked_at.strip() or utc_now_iso()
    report = build_report(
        repo_url=args.repo_url,
        previous_head=previous_head,
        current_head=current_head,
        checked_at=checked_at,
    )

    if args.out_json:
        out_json_path = resolve_path(repo_root, args.out_json)
        out_json_path.parent.mkdir(parents=True, exist_ok=True)
        out_json_path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

    if args.out_markdown:
        out_md_path = resolve_path(repo_root, args.out_markdown)
        out_md_path.parent.mkdir(parents=True, exist_ok=True)
        out_md_path.write_text(render_markdown(report), encoding="utf-8")

    if args.update_state:
        save_state(
            state_path,
            {
                "tracked_repo": args.repo_url,
                "last_known_head": current_head,
                "last_checked_at": checked_at,
            },
        )

    json.dump(report, sys.stdout, ensure_ascii=False)
    sys.stdout.write("\n")

    if args.fail_on_change and report["changed"]:
        print("error: upstream head changed; review report and refresh baseline.", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
