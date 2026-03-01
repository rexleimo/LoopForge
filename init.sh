#!/usr/bin/env bash
set -euo pipefail

echo "[rexos] building..."
cargo build

echo "[rexos] smoke: CLI help"
cargo run -- --help >/dev/null

echo "[rexos] done"

