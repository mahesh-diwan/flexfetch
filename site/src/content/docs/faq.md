---
title: FAQ
description: Frequently asked questions about flexfetch
order: 13
---

## General

**How is this different from neofetch/fastfetch?**
Lua plugins, WASM plugins, Tera templates, and 28 theme presets — no other
tool has all three. Plus live dashboard, smart fetch, health score, and
remote SSH fetch.

**How do I add info that isn't built in?**
Two ways: a `[custom]` config section (shell commands) or a Lua plugin.

**Does it work on macOS?**
Yes. OS detection via `sw_vers`, macOS logo auto-detected, and the release
pipeline builds both arm64 and x86_64 binaries.

**What's the license?**
MIT.

## Installation

**Do prebuilt binaries include Lua plugins?**
No — Releases/install.sh binaries are pure Rust (no Lua) to stay lean.
Source builds include Lua by default.

**How do I update?**
If you installed via the install script: `flexfetch --update`.
Otherwise, re-run the install script or `cargo install --git`.

**Why is my minimal build missing templates / image logos / the live dashboard?**
Those are feature-gated for the binary diet. Build with `--features
live,image-logos,tera` to opt back in.

## Configuration

**Where is the config file?**
`~/.config/flexfetch/config.toml`. Generate a starter with `flexfetch --gen-config`.

**How do I use a custom template?**
Place it in `~/.config/flexfetch/templates/` and use `flexfetch -t my_template`.

**Can I override theme colors?**
Yes. Set `color_keys`, `color_values`, `color_sep` in `[display]`.

**Does config hot-reload work?**
Yes — `--watch` and `--live` detect config changes by mtime and re-apply
them on the next refresh.

## Modules

**How many modules are there?**
38 built-in modules across 5 sections (System, Hardware, Network, Software,
Context & extras).

**What's the difference between static and dynamic modules?**
Static modules are collected once per session and reused in watch/live mode.
Dynamic modules are re-collected every tick.

**Can I write my own module?**
Yes — drop a `.lua` file in `~/.config/flexfetch/plugins/`. See [Plugins](/docs/plugins).

## Troubleshooting

**My terminal shows tofu boxes instead of icons.**
Your terminal doesn't have a Nerd Font. Icons are automatically blanked when
Nerd Font detection fails — the output falls back to plain-text keys.

**`--ssh` fails with "connection refused".**
The remote host must have SSH access configured. If flexfetch isn't installed
on the remote, the current binary is automatically scp'd to `/tmp/` and run
from there.

**`--update-db` returns an error.**
The crowdsourced hardware database requires an internet connection. It falls
back to the bundled seed when offline.