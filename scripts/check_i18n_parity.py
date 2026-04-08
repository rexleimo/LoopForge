#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path


def resolve_repo_root() -> Path:
    return Path(__file__).resolve().parents[1]


def collect_base_markdown_files(docs_root: Path, locale: str) -> list[Path]:
    out: list[Path] = []
    for path in docs_root.rglob("*.md"):
        rel = path.relative_to(docs_root)
        if rel.parts and rel.parts[0] == locale:
            continue
        out.append(rel)
    out.sort()
    return out


def collect_missing_translations(docs_root: Path, locale: str) -> list[Path]:
    missing: list[Path] = []
    for rel in collect_base_markdown_files(docs_root, locale):
        localized = docs_root / locale / rel
        if not localized.is_file():
            missing.append(rel)
    return missing


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Check docs-site locale parity: every EN markdown page should have a localized counterpart."
    )
    parser.add_argument("--docs-root", default="docs-site")
    parser.add_argument("--locale", default="zh-CN")
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args(argv)

    repo_root = resolve_repo_root()
    docs_root = Path(args.docs_root)
    if not docs_root.is_absolute():
        docs_root = repo_root / docs_root

    if not docs_root.is_dir():
        print(f"error: docs root does not exist: {docs_root}", file=sys.stderr)
        return 2

    missing = collect_missing_translations(docs_root, args.locale)

    if args.json:
        payload = {
            "docs_root": str(docs_root),
            "locale": args.locale,
            "missing_count": len(missing),
            "missing": [str(p).replace("\\", "/") for p in missing],
        }
        json.dump(payload, sys.stdout, ensure_ascii=False, indent=2)
        sys.stdout.write("\n")
    else:
        if missing:
            print(
                f"i18n parity check failed for locale `{args.locale}`: {len(missing)} missing pages:"
            )
            for path in missing:
                print(f"- {path.as_posix()}")
        else:
            print(f"i18n parity check passed for locale `{args.locale}`")

    return 1 if missing else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
