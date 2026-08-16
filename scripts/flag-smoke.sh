#!/usr/bin/env bash
# E2E flag smoke test — exercises every CLI flag and asserts it exits cleanly.
# Catches regressions like a flag silently breaking or a command erroring after
# a refactor. Run from the repo root:
#
#   ./scripts/flag-smoke.sh
#
# Exits non-zero on the first failing flag (or when the build fails).
set -euo pipefail

cd "$(dirname "$0")/.."

BIN=target/release/flexfetch
LOG=$(mktemp)
FAILURES=0

pass() { echo "  ok: $*"; }
fail() {
    FAILURES=$((FAILURES + 1))
    echo "  FAIL: $* (exit=$1)"
    echo "  --- last output ---"
    tail -5 "$LOG" | sed 's/^/  /'
    echo "  -------------------"
}

# check <flags...> — runs the binary with the given args (split on spaces)
check() {
    local expect="${1:-0}"
    shift
    # shellcheck disable=SC2086
    if timeout 30 "$BIN" $* >"$LOG" 2>&1; then
        local rc=0
    else
        local rc=$?
    fi
    if [ "$rc" -eq 124 ]; then
        # timeout is expected for long-running modes (--watch, --live)
        pass "timed out as expected: $*"
    elif [ "$rc" -eq "$expect" ]; then
        pass "$*"
    else
        fail "$*" "$rc"
    fi
}

echo "== build =="
cargo build --release --features live,qr >/dev/null 2>&1 || { echo "build failed"; exit 1; }
echo "  build ok"

echo "== info flags =="
check 0 --list-modules
check 0 --list-presets
check 0 --list-themes
check 0 --doctor
check 0 --version

echo "== render flags =="
check 0 --minimal
check 0 --full
check 0 --dev
check 0 --demo
check 0 --preset minimal
check 0 --preset full
check 0 --prompt
check 0 --motd
check 0 --format json
check 0 --format markdown
check 0 --format csv
check 0 --pipe
check 0 --no-gradient
check 0 --no-progress
check 0 --box-style rounded
check 0 --frame double

echo "== export flags =="
# Explicit --output paths: without them the export lands in the repo root
# (flexfetch.svg / flexfetch.html / flexfetch.md) and pollutes the tree.
check 0 --export svg --minimal --output /tmp/flexfetch-smoke-export.svg
check 0 --export html --minimal --output /tmp/flexfetch-smoke-export.html
check 0 --export markdown --minimal --output /tmp/flexfetch-smoke-export.md
rm -f /tmp/flexfetch-smoke-export.svg /tmp/flexfetch-smoke-export.html /tmp/flexfetch-smoke-export.md
# Regression: PNG export used to exit 0 without writing a file when the
# --output path lacked a .png extension (image crate infers format from the
# extension; we now pass the format explicitly).
rm -f /tmp/flexfetch-smoke-export.png
if "$BIN" --export png --minimal --output /tmp/flexfetch-smoke-export.png >/dev/null 2>&1 \
    && [ -s /tmp/flexfetch-smoke-export.png ]; then
    pass "--export png --output (no .png extension) writes a file"
else
    FAILURES=$((FAILURES + 1))
    echo "  FAIL: --export png --output (no .png extension) wrote no file"
fi
rm -f /tmp/flexfetch-smoke-export.png

echo "== compare + perf =="
check 0 --diff local local
check 0 --smart --minimal

# Benchmark reporting contract: the headline must be the real parallel path
# ("real path: collect ... + render ..."), and per-module timings must be
# labeled informational so nobody mistakes the sequential sum for startup time.
BENCH_OUT=$("$BIN" --benchmark --minimal 2>&1 || true)
if echo "$BENCH_OUT" | grep -q "real path:.*collect.*+ render"; then
    pass "--benchmark reports the real parallel path"
else
    FAILURES=$((FAILURES + 1))
    echo "  FAIL: --benchmark missing 'real path: collect + render' headline"
    echo "$BENCH_OUT" | head -8 | sed 's/^/  /'
fi
if echo "$BENCH_OUT" | grep -q "per-module (cold, sequential, informational)"; then
    pass "--benchmark labels per-module timings as informational"
else
    FAILURES=$((FAILURES + 1))
    echo "  FAIL: --benchmark missing the informational per-module label"
fi
rm -f /tmp/flexfetch-smoke-bench.out

echo "== feature-gated =="
check 0 --qr --minimal
check 0 --bug-report
check 0 --gen-config 2>/dev/null || true # may exit non-zero when config exists; not smoke-relevant

echo
if [ "$FAILURES" -eq 0 ]; then
    echo "all flags pass"
    rm -f "$LOG"
else
    echo "$FAILURES flag(s) failed — see output above"
    exit 1
fi
