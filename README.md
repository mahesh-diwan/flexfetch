<p align="center">
  <img src="assets/default.svg" width="660" alt="flexfetch terminal output">
</p>

<h1 align="center">flexfetch</h1>

<p align="center">
  <em>Fast, flexible system info for Linux & macOS · Written in Rust</em>
</p>

<p align="center">
  <a href="#quick-start"><kbd>🚀 Quick Start</kbd></a>
  <a href="#usage"><kbd>📖 Usage</kbd></a>
  <a href="#modules"><kbd>🧩 Modules</kbd></a>
  <a href="#configuration"><kbd>⚙️ Config</kbd></a>
  <a href="#lua-plugins"><kbd>🔌 Lua Plugins</kbd></a>
</p>

<br>

## Quick Start

```bash
git clone https://github.com/mahesh-diwan/flexfetch.git
cd flexfetch
cargo build --release
sudo cp target/release/flexfetch /usr/local/bin/

flexfetch
```

**No dependencies to install** — Rust builds a static binary with everything included. Lua 5.4 optional for plugins.

## Why flexfetch?

|                | flexfetch | neofetch | fastfetch |
| -------------- | --------- | -------- | --------- |
| Language       | Rust      | Bash     | C         |
| Plugins        | Lua 5.4   | —        | —         |
| Templates      | Tera      | —        | —         |
| Config format  | TOML      | —        | JSON5     |
| Binary size    | ~5 MB     | ~1 MB    | ~2 MB     |
| Parallel fetch | ✅        | —        | ✅        |

## Features

- **5 working modules**: OS, Host, Kernel, Uptime, Locale — ready now
- **14 stub modules**: CPU, Memory, Disk, GPU, Network, Battery, Processes, Packages, Shell, Terminal, DE, WM, Colors, Custom — wired up, return empty until implemented
- **Lua plugin system**: drop `.lua` files in `~/.config/flexfetch/plugins/`, get custom info from scripts
- **Tera templates**: Jinja2-like syntax, full control over output layout
- **JSON output**: machine-readable `-f json` for piping into other tools
- **Parallel collection**: modules gather data concurrently via Rayon
- **TOML config**: choose modules, set display options, define custom commands
- **No runtime deps**: static binary, works on any Linux/macOS system

## Usage

```
flexfetch [OPTIONS]
```

| Option                  | What it does                       |
| ----------------------- | ---------------------------------- |
| `-f, --format <FMT>`    | Output: `text` (default) or `json` |
| `-m, --modules <LIST>`  | Colon-separated module list        |
| `-c, --config <FILE>`   | Custom config path                 |
| `-t, --template <NAME>` | Template name (reserved)           |
| `--debug`               | Show per-module errors             |
| `--gen-config`          | Generate default `config.toml`     |
| `--list-modules`        | List available built-in modules    |
| `--list-plugins`        | List discovered Lua plugins        |

### Examples

```bash
# default system info
flexfetch

# machine-readable JSON
flexfetch -f json

# specific modules only
flexfetch -m "os:kernel:uptime"

# pick modules from config
flexfetch -c ~/.config/flexfetch/config.toml

# debug mode to diagnose module errors
flexfetch --debug

# pipe JSON into jq
flexfetch -f json | jq '.os'
```

<p align="center">
  <img src="assets/json.svg" width="660" alt="flexfetch JSON output">
</p>

## Modules

### Working (5)

| Module   | Source                                         | Output                  |
| -------- | ---------------------------------------------- | ----------------------- |
| `os`     | `/etc/os-release` / `sw_vers`                  | name, version, ID, arch |
| `host`   | `gethostname(2)` / `/proc/sys/kernel/hostname` | hostname                |
| `kernel` | `uname -srm`                                   | kernel version + arch   |
| `uptime` | `/proc/uptime` / `sysctl`                      | human-readable uptime   |
| `locale` | `$LANG` / `$LC_CTYPE` / `$LC_ALL`              | language + encoding     |

### Stubs (14 — WIP)

Modules below compile but return empty. They need implementation in `src/modules/`:

`cpu` · `memory` · `disk` · `gpu` · `network` · `battery` · `processes` · `packages` · `shell` · `terminal` · `de` · `wm` · `colors` · `custom`

### Layout-only

`title` — renders header · `separator` — renders divider (template use only)

## Configuration

Config lives at `~/.config/flexfetch/config.toml`:

```toml
modules = ["os", "host", "kernel", "uptime", "colors"]

[display]
separator = ": "
key_width = 8
color_keys = "blue"
color_values = "green"

[cache]
ttl = 60

[custom]
my_custom = { command = "uptime -p", label = "Uptime" }
```

Generate default config: `flexfetch --gen-config`

### Display

| Field          | Default | Description           |
| -------------- | ------- | --------------------- |
| `separator`    | `": "`  | between key and value |
| `key_width`    | `8`     | right-aligns keys     |
| `color_keys`   | —       | ANSI color for keys   |
| `color_values` | —       | ANSI color for values |

### Custom Modules

```toml
[custom]
my_temp = { command = "sensors | grep temp1", label = "Temp" }
sys_load = { command = "uptime | awk -F'load average:' '{print $2}'", label = "Load" }
```

| Field     | Required | Description                     |
| --------- | -------- | ------------------------------- |
| `command` | yes      | shell command to execute        |
| `label`   | no       | display label (default: name)   |
| `shell`   | no       | shell binary (default: `sh -c`) |

## Lua Plugins

Place scripts in `~/.config/flexfetch/plugins/`:

```lua
return {
    name = "my_plugin",
    collect = function(ctx)
        local user = ctx.get_env("USER")
        local host = ctx.run_command("hostname")
        return { value = user .. "@" .. host }
    end
}
```

### Plugin API

| Function      | Signature          | Description           |
| ------------- | ------------------ | --------------------- |
| `read_file`   | `(path) -> string` | read file contents    |
| `run_command` | `(cmd) -> string`  | execute shell command |
| `get_env`     | `(key) -> string`  | get env variable      |

Return a table with `{ value = "..." }` for scalar, or `{ key1 = "val1", key2 = "val2" }` for map.

Built with `mlua` 0.10 (Lua 5.4). Disable: `--no-default-features` on flexfetch-core.

## Templates

Default template uses Tera (Jinja2/Django syntax). Custom templates go in `~/.config/flexfetch/templates/`.

Template variables are per-module keys (`os`, `host`, `kernel`, …) plus `display_separator` and `display_key_width`.

## Output Formats

| Format | Command               | Use case             |
| ------ | --------------------- | -------------------- |
| text   | `flexfetch` (default) | human-readable       |
| json   | `flexfetch -f json`   | programmatic parsing |

## Project Structure

```
flexfetch/
├── Cargo.toml              # workspace manifest
├── flexfetch-core/         # detection library
│   └── src/
│       ├── lib.rs          # crate root + re-exports
│       ├── module.rs       # Module trait, InfoValue, SystemInfo
│       ├── module_registry.rs  # registry + parallel dispatch
│       ├── config.rs       # TOML config loader
│       ├── context.rs      # runtime context (dirs, cache)
│       ├── template.rs     # Tera template engine
│       ├── cache.rs        # file-backed JSON cache (TTL)
│       ├── error.rs        # error types
│       └── modules/        # detection modules
├── flexfetch-cli/          # CLI binary
│   └── src/main.rs
├── flexfetch-lua/          # Lua plugin host (optional)
│   └── src/lib.rs
└── templates/
    └── default.tera
```

## Requirements

- **Rust** 1.75+ (edition 2021)
- **OS**: Linux (primary) or macOS
- **Lua 5.4** (optional, for plugins)

All crate dependencies managed by Cargo: `clap`, `serde`/`serde_json`, `toml`, `tera`, `rayon`, `mlua`, `chrono`, `walkdir`, `libc`.

## License

MIT

## Credits

- [neofetch](https://github.com/dylanaraps/neofetch) — inspiration
- [fastfetch](https://github.com/fastfetch-cli/fastfetch) — Rust reference
- [Tera](https://tera.netlify.app/) — template engine
- [mlua](https://github.com/khvzak/mlua) — Lua bindings
