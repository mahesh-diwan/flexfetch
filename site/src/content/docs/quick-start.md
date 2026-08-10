---
title: Quick Start
description: Common commands and next steps
order: 3
---

Run it with no arguments to fetch the default module set:

```bash
flexfetch
```

## Common commands

| Command                           | What it does                               |
| --------------------------------- | ------------------------------------------ |
| `flexfetch --theme nord`          | Switch theme                               |
| `flexfetch -m "os:kernel:uptime"` | Select modules explicitly                  |
| `flexfetch --preset minimal`      | Use a preset module list                   |
| `flexfetch -f json`               | Structured JSON output                     |
| `flexfetch --list-modules`        | List all built-in modules                  |
| `flexfetch --list-themes`         | List all theme presets                     |
| `flexfetch --gen-config`          | Generate `~/.config/flexfetch/config.toml` |
| `flexfetch --live`                | Real-time dashboard                        |
| `flexfetch --wizard`              | Interactive config wizard                  |
| `flexfetch --prompt`              | One-line shell prompt string               |
| `flexfetch --ssh host`            | Fetch a remote host's info                 |
| `flexfetch --smart`               | Context-aware (git, project, container)    |
| `flexfetch --health`              | System health score (0-100)                |
| `flexfetch completions bash`      | Print bash completions                     |

## What you'll see

After running `flexfetch`, you get a themed ASCII logo of your distro on the
left and system info on the right — OS, kernel, CPU, memory, uptime, packages,
and more. The output is colorized by default (truecolor), with key/value pairs
and small icons from Nerd Fonts.

Example output:

```
    ___          cachyos
   /   \         ──────────────────────────────────
  /     \        OS        CachyOS Linux x86_64
 /  /|  \ |      Kernel    6.12.3-2-cachyos
/  / |   \|      Uptime    4 hours, 12 mins
    |        |   Packages  1847 (pacman)
    |    |  |    Shell     bash 5.2.37
    |    |  |    Terminal  kitty 0.38.0
    |    |  |    CPU       AMD Ryzen 9 7950X (32) @ 5.88 GHz
                        Memory  8.2 GiB / 31.1 GiB (26%)
```

The logo and colors adapt to your theme. Try a few with `flexfetch --theme dracula`
or `flexfetch --theme nord` to see different color schemes.

## Picking modules

The default output includes a curated set of modules, but you can narrow it
down. `-m` takes a colon-separated list:

```bash
flexfetch -m "os:kernel:cpu:memory:uptime"
```

Or use a preset — each one groups modules for a specific use case:

```bash
flexfetch --preset minimal    # just OS + hostname + uptime
flexfetch --preset hardware   # CPU, GPU, memory, disk, network
flexfetch --preset software   # kernel, packages, shell, terminal
flexfetch --preset all        # everything
```

## Output formats

Switch the output format with `-f`:

```bash
flexfetch -f json             # structured JSON for scripts
flexfetch -f yaml             # YAML
flexfetch -f markdown         # Markdown tables
flexfetch -f csv              # CSV (one row per module)
```

JSON output is useful for piping into `jq` or feeding into other tools.

## Next steps

1. **Customize** — run `flexfetch --wizard` for an interactive config builder
2. **Pick a theme** — `flexfetch --list-themes` shows all 28 presets
3. **Select modules** — `flexfetch --list-modules` shows all 38 built-in modules
4. **Try the dashboard** — `flexfetch --live` for real-time monitoring

For the full flag list see the [CLI reference](/docs/output) or `flexfetch --help`.
