#!/usr/bin/env python3
from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import platform
import shlex
import subprocess
import time
from pathlib import Path


def _iso_now() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()


def _env(name: str, env: dict[str, str], default: str = "") -> str:
    return env.get(name, default).strip()


def build_smoke_cases(env: dict[str, str]) -> list[dict[str, object]]:
    ollama_model = _env("LOOPFORGE_OLLAMA_MODEL", env, "qwen3:4b")
    skip_ollama = _env("LOOPFORGE_SKIP_OLLAMA_SMOKE", env).lower() in (
        "1",
        "true",
        "yes",
        "on",
    )
    cases: list[dict[str, object]] = []

    if not skip_ollama:
        cases.append(
            {
                "id": "ollama_smoke",
                "description": "Local Ollama OpenAI-compatible smoke",
                "required_env": [],
                "command": (
                    f"LOOPFORGE_OLLAMA_MODEL={shlex.quote(ollama_model)} "
                    "cargo test --workspace --test ollama_smoke -- --ignored --nocapture"
                ),
            }
        )

    if _env("ZHIPUAI_API_KEY", env):
        model = _env("LOOPFORGE_GLM_MODEL", env, "glm-4")
        cases.append(
            {
                "id": "zhipu_smoke",
                "description": "Zhipu GLM native smoke",
                "required_env": ["ZHIPUAI_API_KEY"],
                "command": (
                    f"LOOPFORGE_GLM_MODEL={shlex.quote(model)} "
                    "cargo test --workspace --test zhipu_smoke -- --ignored --nocapture"
                ),
            }
        )

    if _env("MINIMAX_API_KEY", env):
        model = _env("LOOPFORGE_MINIMAX_MODEL", env, "MiniMax-M2.5")
        cases.append(
            {
                "id": "minimax_smoke",
                "description": "MiniMax native smoke",
                "required_env": ["MINIMAX_API_KEY"],
                "command": (
                    f"LOOPFORGE_MINIMAX_MODEL={shlex.quote(model)} "
                    "cargo test --workspace --test minimax_smoke -- --ignored --nocapture"
                ),
            }
        )

    if _env("NVIDIA_API_KEY", env):
        model = _env("LOOPFORGE_NVIDIA_MODEL", env, "meta/llama-3.2-3b-instruct")
        cases.append(
            {
                "id": "nvidia_smoke",
                "description": "NVIDIA NIM OpenAI-compatible smoke",
                "required_env": ["NVIDIA_API_KEY"],
                "command": (
                    f"LOOPFORGE_NVIDIA_MODEL={shlex.quote(model)} "
                    "cargo test --workspace --test nvidia_nim_smoke -- --ignored --nocapture"
                ),
            }
        )

    if _env("LOOPFORGE_BEDROCK_MODEL", env):
        model = _env("LOOPFORGE_BEDROCK_MODEL", env)
        region = _env("LOOPFORGE_BEDROCK_REGION", env, "us-east-1")
        cases.append(
            {
                "id": "bedrock_smoke",
                "description": "AWS Bedrock Converse API smoke",
                "required_env": ["LOOPFORGE_BEDROCK_MODEL"],
                "command": (
                    f"LOOPFORGE_BEDROCK_REGION={shlex.quote(region)} "
                    f"LOOPFORGE_BEDROCK_MODEL={shlex.quote(model)} "
                    "cargo test -p rexos --features bedrock --test bedrock_smoke -- --ignored --nocapture"
                ),
            }
        )

    return cases


def run_case(case: dict[str, object], cwd: Path, run_commands: bool) -> dict[str, object]:
    required_env = [str(x) for x in case.get("required_env", [])]
    missing = [name for name in required_env if not os.environ.get(name, "").strip()]
    started_at = time.time()
    if missing:
        return {
            "id": case["id"],
            "status": "skipped",
            "duration_sec": 0.0,
            "details": f"missing env: {', '.join(missing)}",
            "command": case["command"],
        }
    if not run_commands:
        return {
            "id": case["id"],
            "status": "planned",
            "duration_sec": 0.0,
            "details": "dry-run (pass --run to execute)",
            "command": case["command"],
        }

    proc = subprocess.run(
        str(case["command"]),
        cwd=str(cwd),
        shell=True,
        capture_output=True,
        text=True,
    )
    duration = round(time.time() - started_at, 2)
    status = "pass" if proc.returncode == 0 else "fail"
    details = f"exit={proc.returncode}"
    if proc.returncode != 0 and proc.stderr.strip():
        details += f"; stderr={proc.stderr.strip().splitlines()[-1]}"
    return {
        "id": case["id"],
        "status": status,
        "duration_sec": duration,
        "details": details,
        "command": case["command"],
    }


def render_markdown(report: dict[str, object]) -> str:
    lines: list[str] = []
    lines.append("# Provider Health Report")
    lines.append("")
    lines.append(f"- Generated: {report.get('generated_at', '')}")
    lines.append(f"- Host: {report.get('host', '')}")
    lines.append(f"- Mode: {report.get('mode', '')}")
    lines.append("")
    lines.append("| Case | Status | Duration(s) | Details |")
    lines.append("|---|---|---:|---|")
    for row in report.get("results", []):
        if not isinstance(row, dict):
            continue
        lines.append(
            "| {id} | {status} | {dur:.2f} | {details} |".format(
                id=row.get("id", ""),
                status=row.get("status", ""),
                dur=float(row.get("duration_sec", 0.0)),
                details=str(row.get("details", "")).replace("|", "/"),
            )
        )
    lines.append("")
    lines.append("## Commands")
    lines.append("")
    for row in report.get("results", []):
        if not isinstance(row, dict):
            continue
        lines.append(f"- `{row.get('id', '')}`: `{row.get('command', '')}`")
    lines.append("")
    return "\n".join(lines)


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Generate provider smoke health report for available credentials."
    )
    parser.add_argument(
        "--out-dir",
        default=".tmp/provider-health",
        help="Output directory for report files (default: .tmp/provider-health)",
    )
    parser.add_argument(
        "--run",
        action="store_true",
        help="Execute smoke commands (default: dry-run planning only)",
    )
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root used as command cwd (default: .)",
    )
    args = parser.parse_args(argv)

    repo_root = Path(args.repo_root).resolve()
    out_dir = Path(args.out_dir).resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    env = dict(os.environ)
    cases = build_smoke_cases(env)
    results = [run_case(case, repo_root, args.run) for case in cases]

    report = {
        "generated_at": _iso_now(),
        "host": platform.node() or "unknown",
        "mode": "run" if args.run else "dry-run",
        "results": results,
    }

    json_path = out_dir / "provider-health.json"
    md_path = out_dir / "provider-health.md"
    json_path.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    md_path.write_text(render_markdown(report) + "\n", encoding="utf-8")

    print(f"wrote: {json_path}")
    print(f"wrote: {md_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(os.sys.argv[1:]))
