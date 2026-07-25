#!/usr/bin/env bash
#
# Run the swiss_qrust benchmark suite (criterion).
#
# Usage:
#   scripts/bench.sh                        # run once, compare against the last saved baseline
#   scripts/bench.sh --save-baseline main   # run and save results under a named baseline
#   scripts/bench.sh --baseline main        # compare against a named baseline instead of the default
#   scripts/bench.sh <criterion args...>    # anything else is forwarded to the bench binary as-is
set -euo pipefail
cd "$(dirname "$0")/.."

cargo bench -p swiss_qrust --bench render_bench -- "$@"

echo
echo "==> HTML report: target/criterion/report/index.html"
