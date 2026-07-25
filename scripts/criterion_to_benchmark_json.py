#!/usr/bin/env python3
"""
Convert criterion's per-benchmark target/criterion/<name>/new/{benchmark,estimates}.json
into the JSON array format benchmark-action/github-action-benchmark expects
for its 'customSmallerIsBetter' tool:

    [{"name": "...", "unit": "ns", "value": 1234.5}, ...]

criterion's own --output-format bencher (as of criterion 0.8) prints the
name and the "bench: N ns/iter" line on separate lines/streams instead of
the combined "test <name> ... bench: N ns/iter" line the action's 'cargo'
parser expects, so that format can't be piped in directly - this reads
criterion's persisted stats instead, which is exact rather than reparsed
CLI text.
"""
import json
import sys
from pathlib import Path

criterion_dir = Path(sys.argv[1] if len(sys.argv) > 1 else "target/criterion")

results = []
for benchmark_json in sorted(criterion_dir.glob("*/new/benchmark.json")):
    estimates_json = benchmark_json.parent / "estimates.json"
    if not estimates_json.exists():
        continue

    name = json.loads(benchmark_json.read_text())["full_id"]
    mean_ns = json.loads(estimates_json.read_text())["mean"]["point_estimate"]

    results.append({"name": name, "unit": "ns", "value": mean_ns})

if not results:
    print(f"no criterion results found under {criterion_dir}", file=sys.stderr)
    sys.exit(1)

json.dump(results, sys.stdout, indent=2)
print()
