# Introduction

**flexfetch** is a fast, flexible system info tool written in Rust. It prints
your system details — OS, kernel, CPU, memory, disks, and more — with a logo
and themeable output.

## What makes it different

|     | Feature              | What it means                                                                                              |
| --- | -------------------- | ---------------------------------------------------------------------------------------------------------- |
| 📝  | **Tera templates**   | Jinja2-style templates. Variables, loops, conditionals. Default template renders side-by-side logo + info. |
| 🎭  | **27 theme presets** | Catppuccin, Dracula, Nord, Gruvbox, Tokyo Night, Rose Pine, and more. Switch with `--theme`.               |
| ⚡  | **Rust + Rayon**     | Parallel detection. Statically linked on Linux; as small as ~1.75 MB in the minimal build.                  |
| 🖥️  | **Live dashboard**   | Real-time CPU/memory gauges with sparklines, top processes, network throughput.                            |
| 🔑  | **Smart fetch**      | Context-relevant info: git branch, project type, container/venv detection.                                 |
| 🏥  | **Health score**     | 0–100 system health from disk, swap, load, and battery.                                                    |
| 🌍  | **Remote fetch**     | Fetch remote system info via SSH. Auto-installs if the host lacks flexfetch.                               |
| 🎨  | **Image logos**      | Kitty, iTerm2, Sixel, and block-unicode image protocols for distro logos.                                  |

## At a glance

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
