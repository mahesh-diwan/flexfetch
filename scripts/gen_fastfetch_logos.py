#!/usr/bin/env python3
"""Generate fastfetch_logos.rs from fastfetch ASCII logo files."""

import os
import sys

LOGO_DIR = "/tmp/fastfetch-logos/src/logo/ascii"
OUTPUT = "/home/mahesh-diwan/SPECTRE/Projects/flexfetch/flexfetch-core/src/fastfetch_logos.rs"


def read_logo(path):
    with open(path) as f:
        return f.read().rstrip("\n")


def main():
    logos = {}  # name -> lines string
    for letter_dir in sorted(os.listdir(LOGO_DIR)):
        letter_path = os.path.join(LOGO_DIR, letter_dir)
        if not os.path.isdir(letter_path):
            continue
        for fname in sorted(os.listdir(letter_path)):
            if not fname.endswith(".txt"):
                continue
            name = fname[:-4]  # strip .txt
            content = read_logo(os.path.join(letter_path, fname))
            if content.strip():
                logos[name] = content

    lines = []
    lines.append("// Auto-generated from fastfetch ASCII logos (MIT license).")
    lines.append("// Source: https://github.com/fastfetch-cli/fastfetch")
    lines.append(
        "// Do not edit manually — run: python3 scripts/gen_fastfetch_logos.py"
    )
    lines.append("")
    lines.append("/// Returns logo lines for fastfetch-sourced distros, or None.")
    lines.append("pub fn fastfetch_logo(name: &str) -> Option<&'static str> {")
    lines.append("    match name {")

    for name, content in sorted(logos.items()):
        # Escape backslashes and double quotes for Rust string literals
        escaped = content.replace("\\", "\\\\").replace('"', '\\"')
        lines.append(f'        "{name}" => Some("{escaped}"),')

    lines.append("        _ => None,")
    lines.append("    }")
    lines.append("}")

    with open(OUTPUT, "w") as f:
        f.write("\n".join(lines) + "\n")

    print(f"Generated {len(logos)} logos → {OUTPUT}")


if __name__ == "__main__":
    main()
