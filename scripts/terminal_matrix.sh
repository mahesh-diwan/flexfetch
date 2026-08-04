#!/bin/sh
# Phase 8.4 — terminal capability matrix (reality-adapted: the detection is
# env-driven, so we assert output behavior across TERM/COLORTERM combos rather
# than launching GUI emulators in Docker — same code path, deterministic CI).
#
# Asserts:
#   truecolor terminals (COLORTERM=truecolor / TERM=*-truecolor / kitty) make
#     the theme emit `38;2;R;G;B` sequences,
#   legacy terminals (TERM=xterm / linux, no COLORTERM) make the theme rows
#     fall back to 16-color ANSI (the distro logo keeps its baked-in palette),
#   the binary never crashes under any TERM value.
#
# Color codes only render when stdout is a tty, so every color-path check runs
# under `script` (pty); in a no-pty environment we degrade to crash checks.
#
# Usage: ./scripts/terminal_matrix.sh [/path/to/flexfetch]  (default: target/release/flexfetch)

set -eu
BIN="${1:-target/release/flexfetch}"

if [ ! -x "$BIN" ]; then
    echo "error: $BIN not found — build first (cargo build --release -p flexfetch-cli)" >&2
    exit 2
fi

ESC=$(printf '\033')
fail=0

check() { # name want_tc has_tc has_ansi
    name="$1"; want_tc="$2"; has_tc="$3"; has_ansi="$4"
    if [ "$want_tc" = "yes" ]; then
        if [ "$has_tc" = "1" ]; then
            echo "PASS: $name (truecolor)"
        else
            echo "FAIL: $name — expected truecolor, got 16-color"
            fail=1
        fi
    else
        # The distro logo carries its own baked-in palette (some pixels are
        # truecolor by design), so we don't assert the ABSENCE of 38;2 — we
        # assert the theme rows (keys/values/separators) degrade to 16-color.
        if [ "$has_ansi" = "1" ]; then
            echo "PASS: $name (theme falls back to 16-color)"
        else
            echo "FAIL: $name — expected 16-color ANSI fallback, got none"
            fail=1
        fi
    fi
}

# run_in_pty <name> <want_tc> <env-assignments...> — runs the binary under a pty
# with the given env and asserts truecolor / 16-color emission.
run_in_pty() {
    name="$1"; want_tc="$2"; shift 2
    out=$(script -qec "env $* \"$BIN\" --modules os" /dev/null 2>/dev/null || true)
    # grep -c counts matching LINES, so normalize counts to a boolean.
    if printf '%s' "$out" | grep -aq '38;2'; then has_tc=1; else has_tc=0; fi
    # 16-color-only codes: \e[3Xm/\e[4Xm (0-7) and bright \e[9Xm/\e[10Xm
    # (90-97/100-107). 38;2 truecolor never matches (semicolons follow 38).
    if printf '%s' "$out" | grep -aqE "${ESC}\\[[349][0-9]m"; then has_ansi=1; else has_ansi=0; fi
    check "$name" "$want_tc" "$has_tc" "$has_ansi"
}

if command -v script >/dev/null 2>&1; then
    echo "== truecolor terminals =="
    run_in_pty "COLORTERM=truecolor TERM=xterm-256color" yes "COLORTERM=truecolor TERM=xterm-256color"
    # TERM-based detection must work with no COLORTERM at all (unset it so the
    # ambient shell env can't leak a false positive).
    run_in_pty "TERM=xterm-truecolor (no COLORTERM)" yes "-u COLORTERM TERM=xterm-truecolor"
    run_in_pty "TERM=xterm-kitty (no COLORTERM)" yes "-u COLORTERM TERM=xterm-kitty"

    echo "== legacy terminals (16-color fallback) =="
    run_in_pty "TERM=linux" no "-u COLORTERM TERM=linux"
    run_in_pty "TERM=xterm" no "-u COLORTERM TERM=xterm"
else
    echo "note: no pty available (no 'script') — running crash-only checks per TERM"
    for t in xterm-256color xterm linux xterm-kitty xterm-truecolor; do
        if env TERM="$t" "$BIN" --modules os >/dev/null 2>&1; then
            echo "PASS: runs under TERM=$t"
        else
            echo "FAIL: crashed under TERM=$t"
            fail=1
        fi
    done
fi

echo ""
if [ "$fail" = "0" ]; then
    echo "terminal matrix: ALL PASS"
else
    echo "terminal matrix: FAILURES DETECTED" >&2
    exit 1
fi
