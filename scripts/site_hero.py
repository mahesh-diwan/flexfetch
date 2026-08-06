#!/usr/bin/env python3
"""Regenerate the site hero terminal render from a REAL flexfetch run.

Runs the built binary in a PTY at a fixed size (so the layout is
deterministic), converts the ANSI SGR escapes to inline-styled HTML spans,
and writes the fragment to site/assets/hero.html.

Usage:  python3 scripts/site_hero.py   (after `cargo build --release -p flexfetch-cli`)
"""
import html
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "site" / "assets" / "hero.html"
BIN = ROOT / "target" / "release" / "flexfetch"

COLS, ROWS = 92, 40


def capture() -> str:
    if not BIN.exists():
        sys.exit(f"error: {BIN} not found — run `cargo build --release -p flexfetch-cli` first")
    with tempfile.NamedTemporaryFile(suffix=".typescript", delete=False) as f:
        ts = f.name
    try:
        cmd = f"stty cols {COLS} rows {ROWS}; {BIN}"
        env = {**os.environ, "TERM": "xterm-256color"}
        subprocess.run(
            ["script", "-qec", cmd, ts],
            env=env, cwd=ROOT, capture_output=True,
        )
        return Path(ts).read_text(encoding="utf-8", errors="replace")
    finally:
        os.unlink(ts)


def convert(raw: str) -> str:
    """ANSI SGR → HTML spans. Keeps only foreground/background color + bold."""
    lines = raw.splitlines()
    if lines and lines[0].startswith("Script started"):
        lines = lines[1:]
    if lines and lines[-1].startswith("Script done"):
        lines = lines[:-1]
    text = "\n".join(lines)

    out: list[str] = []
    stack: list[str] = []
    i = 0
    n = len(text)

    def close() -> None:
        while stack:
            out.append("</span>")
            stack.pop()

    def push(style: str) -> None:
        out.append(f'<span style="{style}">')
        stack.append(style)

    while i < n:
        c = text[i]
        if c == "\x1b":
            m = re.match(r"\x1b\[([0-9;]*)m", text[i:])
            if m:
                params = m.group(1)
                if params in ("", "0"):
                    close()
                else:
                    # SGR params can be multi-token (38;2;R;G;B spans 5 tokens), so
                    # walk them with an index instead of per-token matches.
                    style: list[str] = []
                    toks = params.split(";")
                    j = 0
                    while j < len(toks):
                        t = toks[j]
                        if t == "1":
                            style.append("font-weight:600")
                        elif t == "38" and j + 4 < len(toks) and toks[j + 1] == "2":
                            style.append(f"color:rgb({toks[j+2]},{toks[j+3]},{toks[j+4]})")
                            j += 4
                        elif t == "48" and j + 4 < len(toks) and toks[j + 1] == "2":
                            style.append(f"background:rgb({toks[j+2]},{toks[j+3]},{toks[j+4]})")
                            j += 4
                        j += 1
                    if style:
                        push(";".join(style))
                i += len(m.group(0))
                continue
            m = re.match(r"\x1b\[[0-9;?]*[A-Za-z]", text[i:])
            if m:
                i += len(m.group(0))
                continue
            i += 1
            continue
        out.append(html.escape(c))
        i += 1
    close()
    return "".join(out)


def main() -> None:
    raw = capture()
    frag = convert(raw)
    lines = frag.split("\n")
    # Trim trailing blank lines so the terminal body has no dead space.
    while lines and not lines[-1].strip():
        lines.pop()
    frag = "\n".join(lines)
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(frag + "\n", encoding="utf-8")
    print(f"wrote {OUT} ({len(frag)} chars, {len(lines)} lines)")


if __name__ == "__main__":
    main()
