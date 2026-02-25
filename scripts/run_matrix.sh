#!/usr/bin/env bash
# Run portable benchmarks under multiple iteration settings.
set -euo pipefail
ITERS_LIST=("1000" "10000" "50000")
for it in "${ITERS_LIST[@]}"; do
  echo "== iters=$it =="
  cargo run --release -- run --preset core --iters "$it" --json "reports/core_${it}.json"
done
