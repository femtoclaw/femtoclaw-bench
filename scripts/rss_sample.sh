#!/usr/bin/env bash
# Sample RSS of a running process (Linux/macOS best-effort)
# Usage: ./scripts/rss_sample.sh <pid> <seconds>
set -euo pipefail
PID="${1:-}"
SECS="${2:-10}"
if [ -z "$PID" ]; then
  echo "usage: $0 <pid> <seconds>"
  exit 1
fi
for i in $(seq 1 "$SECS"); do
  if ps -p "$PID" >/dev/null 2>&1; then
    # RSS in KB (ps output varies slightly across OS)
    ps -o rss= -p "$PID" | tr -d ' ' | awk '{print "t=" NR " rss_kb=" $1}'
    sleep 1
  else
    echo "process exited"
    exit 0
  fi
done
