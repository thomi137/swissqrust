#!/usr/bin/env bash
#
# Pretty test report for the swiss_qrust library crate.
#
# Runs unit + integration tests through cargo-nextest (which gives the
# colored, per-test tree output) and then doc tests separately, since
# nextest doesn't run those (https://github.com/nextest-rs/nextest/issues/16).
#
# Usage:
#   scripts/test-report.sh              # human-readable run
#   scripts/test-report.sh --ci         # also writes a JUnit XML report
#                                        # (see .config/nextest.toml [profile.ci])
#   scripts/test-report.sh <nextest args...>   # forwarded to `cargo nextest run`,
#                                                # e.g. -- test_swico to filter
set -euo pipefail
cd "$(dirname "$0")/.."

profile_args=()
if [[ "${1:-}" == "--ci" ]]; then
    profile_args=(--profile ci)
    shift
fi

echo "==> Unit + integration tests (cargo nextest)"
cargo nextest run --package swiss_qrust ${profile_args[@]+"${profile_args[@]}"} "$@"

echo
echo "==> Doc tests (cargo test --doc; nextest doesn't run these)"
cargo test --doc --package swiss_qrust

if [[ "${profile_args[*]:-}" == *ci* ]]; then
    echo
    echo "==> JUnit report: target/nextest/ci/junit.xml"
fi
