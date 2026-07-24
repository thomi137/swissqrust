#!/usr/bin/env bash
#
# Test-coverage report for the swiss_qrust library crate, via cargo-llvm-cov.
#
# Runs the nextest suite and the doc tests under coverage instrumentation
# and merges both into one report (nextest alone can't run doc tests, see
# test-report.sh) - this is llvm-cov's documented pattern for combining the
# two: https://github.com/taiki-e/cargo-llvm-cov#combine-with-cargo-nextest
#
# Usage:
#   scripts/coverage.sh              # open an HTML report in the browser
#   scripts/coverage.sh --summary    # print a per-file summary table only
#   scripts/coverage.sh --lcov       # write target/llvm-cov/lcov.info (for editor gutters/CI)
set -euo pipefail
cd "$(dirname "$0")/.."

cargo llvm-cov clean --package swiss_qrust

echo "==> Unit + integration tests (cargo nextest, instrumented)"
cargo llvm-cov nextest --package swiss_qrust --no-report

# Doc-test coverage needs `--doctests`, which is unstable and only accepted
# by a nightly rustc (it passes -Z unstable-options/--persist-doctests under
# the hood) - fails outright on the stable-only toolchain this repo builds
# with otherwise. Fold it in only if nightly happens to be installed.
if rustup toolchain list 2>/dev/null | grep -q '^nightly'; then
    echo
    echo "==> Doc tests (instrumented via nightly; unstable llvm-cov flag)"
    cargo +nightly llvm-cov --package swiss_qrust --doctests --no-report
else
    echo
    echo "==> Skipping doc-test coverage: needs a nightly toolchain" \
         "(rustup toolchain install nightly). Report below covers lib + integration tests only."
fi

case "${1:-}" in
    --summary)
        echo
        echo "==> Coverage summary"
        cargo llvm-cov report --package swiss_qrust
        ;;
    --lcov)
        cargo llvm-cov report --package swiss_qrust --lcov --output-path target/llvm-cov/lcov.info
        echo
        echo "==> lcov report: target/llvm-cov/lcov.info"
        ;;
    *)
        cargo llvm-cov report --package swiss_qrust --html --open
        echo
        echo "==> HTML report: target/llvm-cov/html/index.html"
        ;;
esac
