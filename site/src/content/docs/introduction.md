---
title: Introduction
description: What is flexfetch and what makes it different
order: 1
---

**flexfetch** is a fast, flexible system info tool written in Rust. It prints
your system details — OS, kernel, CPU, memory, disks, and more — with a logo
and themeable output.

## What makes it different

|     | Feature              | What it means                                                                                              |
| --- | -------------------- | ---------------------------------------------------------------------------------------------------------- |
| 📝  | **Tera templates**   | Jinja2-style templates. Variables, loops, conditionals. Default template renders side-by-side logo + info. |
| 🎭  | **28 theme presets** | Catppuccin, Dracula, Nord, Gruvbox, Tokyo Night, Rose Pine, and more. Switch with `--theme`.               |
| ⚡  | **Rust + Rayon**     | Parallel detection. Static binary, zero runtime deps. As small as ~1.7 MB in the minimal build.            |
| 🖥️  | **Live dashboard**   | Real-time CPU/memory gauges with sparklines, top processes, network throughput.                            |
| 🔑  | **Smart fetch**      | Context-relevant info: git branch, project type, container/venv detection.                                 |
| 🏥  | **Health score**     | 0–100 system health from disk, swap, load, and battery.                                                    |
| 🌍  | **Remote fetch**     | Fetch remote system info via SSH. Auto-installs if the host lacks flexfetch.                               |
| 🎨  | **Image logos**      | Kitty, iTerm2, Sixel, and block-unicode image protocols for distro logos.                                  |

## How it works

flexfetch is a single static binary with no subprocess calls. Every system
detail — CPU model, memory usage, disk stats, network interfaces — is read
directly from `/proc` and `/sys` on Linux, and from `sysctl`/`sw_vers` on
macOS. This means zero fork overhead, no reliance on external tools like
`lscpu` or `free`, and consistently fast execution.

The binary is compiled with Rayon for parallel module detection — CPU, memory,
disk, and network info are gathered concurrently, then assembled into the
output layout. Typical cold-run times are under 5 ms on modern hardware.

### vs. neofetch

neofetch is a shell script that spawns dozens of subprocesses per run. It's
configurable but slow (50–200 ms), unmaintained since 2023, and lacks modern
features like structured output or live dashboards. flexfetch reads `/proc`
directly with zero subprocesses, runs 10–40× faster, and adds Tera
templates and 28 theme presets.

### vs. fastfetch

fastfetch is the closest competitor — also a C binary reading `/proc`
directly. flexfetch matches it on raw speed while adding Tera templates
(Jinja2-style with loops and conditionals), a real-time dashboard
(`--live`), context-aware smart fetch, health scoring, and remote SSH
fetch.

## Requirements

- Linux or macOS
- `curl` for the install script
- A terminal with UTF-8 support

## Source

```bash
flexfetch                          # default output
flexfetch --theme dracula          # switch theme
flexfetch -m "os:kernel:uptime"    # select modules
flexfetch -f json                  # JSON output
flexfetch --live                   # real-time dashboard
flexfetch --smart                  # context-aware (git, project, container)
flexfetch --ssh server1            # remote fetch
```

## Requirements

- Linux or macOS
- `curl` for the install script
- A terminal with UTF-8 support

## Source

The source lives at [github.com/mahesh-diwan/flexfetch](https://github.com/mahesh-diwan/flexfetch).
