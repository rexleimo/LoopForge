#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
import unicodedata
from pathlib import Path

URL_RE = re.compile(r"https?://[^\s]+")


def split_blocks(text: str, separator: str = "---") -> list[str]:
    blocks: list[str] = []
    buf: list[str] = []

    for raw in text.splitlines():
        if raw.strip() == separator:
            block = "\n".join(buf).strip()
            if block:
                blocks.append(block)
            buf = []
            continue
        buf.append(raw)

    tail = "\n".join(buf).strip()
    if tail:
        blocks.append(tail)
    return blocks


def _char_weight(ch: str) -> int:
    if ch == "\r":
        return 0
    if ch == "\n":
        return 1
    return 2 if unicodedata.east_asian_width(ch) in {"W", "F"} else 1


def weighted_length(text: str, url_weight: int = 23) -> int:
    total = 0
    cursor = 0
    for m in URL_RE.finditer(text):
        start, end = m.span()
        segment = text[cursor:start]
        total += sum(_char_weight(ch) for ch in segment)
        total += url_weight
        cursor = end
    total += sum(_char_weight(ch) for ch in text[cursor:])
    return total


def evaluate_posts(
    posts: list[tuple[str, str]],
    limit: int,
    warn_at: int,
    url_weight: int,
) -> list[dict[str, object]]:
    rows: list[dict[str, object]] = []
    for label, text in posts:
        length = weighted_length(text, url_weight=url_weight)
        if length > limit:
            status = "OVER"
        elif length > warn_at:
            status = "WARN"
        else:
            status = "OK"
        rows.append(
            {
                "label": label,
                "length": length,
                "limit": limit,
                "warn_at": warn_at,
                "status": status,
            }
        )
    return rows


def _posts_from_file(path: Path) -> list[tuple[str, str]]:
    blocks = split_blocks(path.read_text(encoding="utf-8"))
    return [(f"{path}#{i + 1}", block) for i, block in enumerate(blocks)]


def _parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Lint X/Twitter post drafts for character limits. "
            "Split file input by lines containing only '---'."
        )
    )
    parser.add_argument(
        "--file",
        action="append",
        default=[],
        help="Path to a text/markdown file. Blocks are split by '---'.",
    )
    parser.add_argument(
        "--post",
        action="append",
        default=[],
        help="Inline post text (can be repeated).",
    )
    parser.add_argument(
        "--limit",
        type=int,
        default=280,
        help="Hard character limit (default: 280).",
    )
    parser.add_argument(
        "--warn-at",
        type=int,
        default=260,
        help="Warning threshold (default: 260).",
    )
    parser.add_argument(
        "--url-weight",
        type=int,
        default=23,
        help="Per-URL character weight (default: 23).",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Output JSON rows instead of plain text.",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = _parse_args(argv)
    limit = max(1, int(args.limit))
    warn_at = max(0, min(limit, int(args.warn_at)))
    url_weight = max(0, int(args.url_weight))

    posts: list[tuple[str, str]] = []
    for file_arg in args.file:
        path = Path(file_arg)
        if not path.exists():
            raise FileNotFoundError(f"file not found: {path}")
        posts.extend(_posts_from_file(path))

    for i, inline_text in enumerate(args.post, start=1):
        text = str(inline_text).strip()
        if text:
            posts.append((f"inline#{i}", text))

    if not posts:
        print("No posts found. Use --file or --post.")
        return 2

    rows = evaluate_posts(posts=posts, limit=limit, warn_at=warn_at, url_weight=url_weight)

    if args.json:
        print(json.dumps(rows, ensure_ascii=False, indent=2))
    else:
        print(f"{'status':<6} {'length':>6}  label")
        for row in rows:
            print(f"{row['status']:<6} {int(row['length']):>6}  {row['label']}")

    has_over = any(str(row["status"]) == "OVER" for row in rows)
    return 1 if has_over else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
