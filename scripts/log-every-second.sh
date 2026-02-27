#!/usr/bin/env bash
# Placeholder script that logs every second (for CI check/lint).
set -e
count=0
max=${1:-300}
while [ "$count" -lt "$max" ]; do
  count=$((count + 1))
  echo "[$(date -u +%H:%M:%S)] tick $count"
  sleep 1
done
echo "Done after $max ticks."
