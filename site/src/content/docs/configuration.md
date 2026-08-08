---
title: Configuration
description: Config reference, cache, presets, and hot reload
order: 4
---

Config lives at `~/.config/flexfetch/config.toml`. Generate a starter with:

```bash
flexfetch --gen-config
```

## Config reference

### Top-level options

| Option        | Type       | Default                        | Description                                               |
| ------------- | ---------- | ------------------------------ | --------------------------------------------------------- |
| `modules`     | `[]string` | `["title","separator",...]`    | Module list to display                                    |
| `template`    | `string`   | `"default"`                    | Template name (looks in `~/.config/flexfetch/templates/`) |
| `plugins_dir` | `string`   | `~/.config/flexfetch/plugins/` | Directory for Lua/WASM plugins                            |

### `[display]` section

| Option            | Type       | Default        | Description                                                  |
| ----------------- | ---------- | -------------- | ------------------------------------------------------------ |
| `separator`       | `string`   | `": "`         | Key-value separator                                          |
| `key_width`       | `integer`  | `8`            | Minimum key column width                                     |
| `theme`           | `string`   | `"catppuccin"` | Theme preset name (or `"random"`)                            |
| `color_title`     | `string`   | —              | Override title color (named color or ANSI escape)            |
| `color_keys`      | `string`   | —              | Override key color                                           |
| `color_values`    | `string`   | —              | Override value color                                         |
| `color_sep`       | `string`   | —              | Override separator color                                     |
| `gradient`        | `boolean`  | `false`        | Enable gradient on keys                                      |
| `gradient_colors` | `[]string` | —              | Custom gradient colors                                       |
| `gradient_title`  | `boolean`  | `true`         | Per-character gradient on the title                          |
| `logo_gradient`   | `boolean`  | `true`         | Per-line gradient on the ASCII logo                          |
| `logo_mode`       | `string`   | `"ascii"`      | Logo mode: `ascii`, `block`, `image`                         |
| `progress_bars`   | `boolean`  | `true`         | Show progress bars for disk/swap/battery                     |
| `palette_style`   | `string`   | `"blocks"`     | Color palette style: `blocks`, `squares`, `dots`             |
| `box_style`       | `string`   | `"rounded"`    | Box drawing: `rounded`, `double`, `dotted`, `thick`, `ascii` |
| `frame`           | `string`   | `"none"`       | Frame style: `none`, `single`, `double`                      |
| `sections`        | `boolean`  | `true`         | Show section headers (System/Hardware/etc)                   |

### `[custom]` section

Define inline info sources that run a shell command on every fetch:

```toml
[custom]
my_temp = { command = "sensors | grep temp1", label = "Temp" }
sys_load = { command = "uptime | awk -'load average:' '{print $2}'", label = "Load" }
```

### `[[modules_config]]` section

Per-module color overrides:

```toml
[[modules_config]]
[modules_config.cpu]
color_keys = "yellow"
color_values = "green"
```

## Full example

```toml
modules = ["title", "separator", "os", "host", "kernel", "uptime",
           "shell", "cpu", "memory", "colors"]

[display]
separator = ": "
key_width = 8
theme = "catppuccin"
box_style = "rounded"
frame = "none"
progress_bars = true
gradient_title = true
palette_style = "blocks"

[cache]
ttl = 60
```

## Cache

Cache is a JSON file at `~/.cache/flexfetch/`. Reduces repeated disk reads.
TTL = 60s by default; `0` disables caching.

## Presets

Built-in presets: `default`, `minimal`, `full`, `dev`, `server`, `laptop`,
`ci`, `neofetch`. User presets live in `~/.config/flexfetch/presets/<name>.toml`
as a `modules = [...]` array:

```bash
flexfetch --preset server
flexfetch --list-presets
```

## Hot reload

`--watch` and `--live` detect changes to the config file (by mtime) and
re-apply them on the next refresh — no restart needed.