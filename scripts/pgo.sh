#!/bin/sh
# PGO (Profile-Guided Optimization) build pipeline.
#
# Builds a release binary instrumented for profile collection, runs it through
# a mix of representative workloads, merges the resulting profiles, then
# rebuilds with -Cprofile-use so the compiler orders code around the hottest
# paths.
#
# Phases:
#   1. profile-generate build   RUSTFLAGS="-Cprofile-generate=$PGO_DIR"
#   2. instrumented runs: --minimal (timed), default, --format json, and
#      --flash if this binary supports it — ~PROFILE_RUNS each
#   3. merge profiles           llvm-profdata merge -> merged.profdata
#   4. profile-use rebuild      RUSTFLAGS="-Cprofile-use=..."
# Then it prints before/after binary sizes and a --minimal timing comparison
# (a plain date-based loop — hyperfine is not required).
#
# RUSTFLAGS is part of cargo's build fingerprint, so phases 1 and 4 each force
# a full recompile — no `touch src/main.rs` / `cargo clean` needed.
#
# A workload whose flag the binary doesn't support is skipped with a warning,
# so the script works against older binaries (e.g. no --flash yet).
#
# Env overrides:
#   PGO_DIR         profile output dir (default /tmp/pgo)
#   PROFILE_RUNS    runs per workload (default 100)
#
# Usage: ./scripts/pgo.sh
# Requires: cargo, llvm-profdata (Debian/Ubuntu: `apt install llvm`).

set -eu

PGO_DIR="${PGO_DIR:-/tmp/pgo}"
PROFILE_RUNS="${PROFILE_RUNS:-100}"
BIN="target/release/flexfetch"

if [ -z "$PGO_DIR" ] || [ "$PGO_DIR" = "/" ]; then
	echo "error: PGO_DIR must be a directory, not '$PGO_DIR'" >&2
	exit 1
fi

command -v cargo >/dev/null 2>&1 || {
	echo "error: cargo not found" >&2
	exit 1
}
command -v llvm-profdata >/dev/null 2>&1 || {
	echo "error: llvm-profdata not found — install the LLVM toolchain" >&2
	echo "  Debian/Ubuntu: apt install llvm" >&2
	exit 1
}

# Timestamps: %s%N (ns) where the date implementation supports it, else %s (s).
NS=0
if ns=$(date +%s%N 2>/dev/null); then
	case "$ns" in
	*[!0-9]*) NS=0 ;; # BSD date prints %N literally
	*) NS=1 ;;
	esac
fi

tick() {
	if [ "$NS" = 1 ]; then date +%s%N; else date +%s; fi
}

elapsed_ms() { # start end
	if [ "$NS" = 1 ]; then
		echo $((($2 - $1) / 1000000))
	else
		echo $((($2 - $1) * 1000))
	fi
}

bin_size() { # bytes — GNU stat first, BSD stat second
	if stat -c %s "$BIN" 2>/dev/null; then
		:
	elif stat -f %z "$BIN" 2>/dev/null; then
		:
	else
		echo 0
	fi
}

time_loop() { # n [args...] — n runs of the binary, prints elapsed ms
	n="$1"
	shift
	start=$(tick)
	i=0
	while [ "$i" -lt "$n" ]; do
		"$BIN" "$@" >/dev/null 2>&1 || true
		i=$((i + 1))
	done
	end=$(tick)
	elapsed_ms "$start" "$end"
}

run_workload() { # label [args...] — PROFILE_RUNS runs; skips unsupported flags
	label="$1"
	shift
	if ! "$BIN" "$@" >/dev/null 2>&1; then
		echo "warning: skipping workload '$label' — flag(s) not supported by this binary"
		return 0
	fi
	time_loop "$PROFILE_RUNS" "$@" >/dev/null
	echo "  $label: collected $PROFILE_RUNS profiles"
}

echo "==> cleaning $PGO_DIR"
rm -rf "$PGO_DIR"
mkdir -p "$PGO_DIR"

echo "==> phase 1/4: profile-generate build"
RUSTFLAGS="-Cprofile-generate=$PGO_DIR" cargo build --release
before_size=$(bin_size)

echo "==> phase 2/4: instrumented runs ($PROFILE_RUNS each)"
if ! "$BIN" --minimal >/dev/null 2>&1; then
	echo "error: binary doesn't support --minimal" >&2
	exit 1
fi
before_ms=$(time_loop "$PROFILE_RUNS" --minimal)
echo "  --minimal: $PROFILE_RUNS runs in $before_ms ms (baseline)"
run_workload default
run_workload "--format json"
run_workload "--flash"

echo "==> phase 3/4: merging profiles"
n=0
for f in "$PGO_DIR"/*.profraw; do
	[ -e "$f" ] || continue
	n=$((n + 1))
done
if [ "$n" -eq 0 ]; then
	echo "error: no .profraw files in $PGO_DIR — instrumented runs produced no data" >&2
	exit 1
fi
echo "  merging $n profile file(s)"
llvm-profdata merge -o "$PGO_DIR/merged.profdata" "$PGO_DIR"/*.profraw

echo "==> phase 4/4: profile-use build"
RUSTFLAGS="-Cprofile-use=$PGO_DIR/merged.profdata" cargo build --release
after_size=$(bin_size)
after_ms=$(time_loop "$PROFILE_RUNS" --minimal)

echo ""
echo "==> summary"
echo "  binary size: before=${before_size}B after=${after_size}B diff=$((after_size - before_size))B"
if [ "$before_ms" -gt 0 ] && [ "$after_ms" -gt 0 ]; then
	speedup=$(awk -v b="$before_ms" -v a="$after_ms" 'BEGIN { printf "%.2f", b / a }')
	echo "  --minimal ($PROFILE_RUNS runs): before=${before_ms}ms after=${after_ms}ms speedup=${speedup}x"
else
	echo "  --minimal ($PROFILE_RUNS runs): before=${before_ms}ms after=${after_ms}ms (timing resolution too coarse)"
fi
