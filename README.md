# flexfetch

Fast, flexible system information tool for Linux and macOS. Written in Rust.

```
$ flexfetch
OS: CachyOS
Kernel: Linux 7.1.3-2-cachyos x86_64
Host: cachyos-x8664
Uptime: 5h 49m
```

## Requirements

- **Rust** 1.75+ (edition 2021)
- **OS**: Linux (primary) or macOS
- **Lua 5.4** (optional, for Lua plugins — enabled by default in CLI build)

### Dependencies (auto-managed by Cargo)

| Crate                  | Purpose                      |
| ---------------------- | ---------------------------- |
| `clap` 4               | CLI argument parsing         |
| `serde` / `serde_json` | JSON serialization           |
| `toml` 0.8             | Config file parsing          |
| `tera` 1               | Template engine              |
| `rayon` 1              | Parallel module collection   |
| `mlua` 0.10            | Lua 5.4 scripting (optional) |
| `chrono` 0.4           | Timestamps                   |
| `walkdir` 2            | Directory traversal          |
| `libc` 0.2             | macOS syscalls               |

## Installation

### From source

```bash
git clone https://github.com/mahesh-diwan/flexfetch.git
cd flexfetch
cargo build --release
sudo cp target/release/flexfetch /usr/local/bin/flexfetch
```

Or run directly:

```bash
cargo run --release
```

### Cargo install (once published)

```bash
cargo install flexfetch
```

## Usage

```
flexfetch [OPTIONS]

Options:
  -c, --config <FILE>      Path to config file
  -m, --modules <LIST>     Colon-separated module list (e.g. "os:cpu:memory")
  -t, --template <NAME>    Template name (reserved, WIP)
  -f, --format <FMT>       Output format: text (default) or json
      --debug              Show module errors
      --gen-config         Generate default config file
      --list-modules       List available built-in modules
      --list-plugins       List loaded Lua plugins
  -h, --help               Print help
  -V, --version            Print version
```

### Examples

```bash
# Basic system info
flexfetch

# JSON output
flexfetch -f json

# Specific modules
flexfetch -m "os:kernel:uptime:packages"

# Generate default config
flexfetch --gen-config

# Debug mode — shows module errors
flexfetch --debug
```

## Configuration

Config file at `~/.config/flexfetch/config.toml`:

```toml
modules = ["os", "host", "kernel", "uptime", "cpu", "memory", "disk", "colors"]

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

## Modules

### Currently implemented

| Module   | Source                                                     | Output                             |
| -------- | ---------------------------------------------------------- | ---------------------------------- |
| `os`     | `/etc/os-release` (Linux), `sw_vers` (macOS)               | OS name, version, ID, architecture |
| `host`   | `/proc/sys/kernel/hostname` (Linux), `gethostname` (macOS) | Hostname                           |
| `kernel` | `uname -srm`                                               | Kernel version + architecture      |
| `uptime` | `/proc/uptime` (Linux), `sysctl` (macOS)                   | Human-readable uptime              |
| `locale` | `$LANG`, `$LC_CTYPE` / `$LC_ALL`                           | System language + encoding         |

### Stubs (WIP — not yet collecting data)

Modules marked `Stub — Task 4/5` in source. They compile but return empty output until implemented:

cpu, memory, disk, gpu, network, battery, processes, packages, shell, terminal, de, wm, colors, custom

### Layout-only (no data collection)

| Name        | Purpose                            |
| ----------- | ---------------------------------- |
| `title`     | Renders title/header in template   |
| `separator` | Renders separator line in template |

## Custom Modules

Define custom commands in config:

```toml
[custom]
my_temp = { command = "sensors | grep temp1", label = "Temp" }
sys_load = { command = "uptime | awk -F'load average:' '{print $2}'", label = "Load" }
```

Fields:

| Field     | Required | Description                             |
| --------- | -------- | --------------------------------------- |
| `command` | Yes      | Shell command to execute                |
| `label`   | No       | Display label (defaults to module name) |
| `shell`   | No       | Shell binary (default: `sh -c`)         |

## Lua Plugins

Place Lua scripts in `~/.config/flexfetch/plugins/`:

```lua
return {
    name = "my_plugin",
    collect = function(ctx)
        local user = ctx.get_env("USER")
        local host = ctx.run_command("hostname")
        return { value = user .. "@" .. host, type = "scalar" }
    end
}
```

### Plugin API (`ctx` table)

| Function      | Signature                  | Description              |
| ------------- | -------------------------- | ------------------------ |
| `read_file`   | `(path: string) -> string` | Read file contents       |
| `run_command` | `(cmd: string) -> string`  | Execute shell command    |
| `get_env`     | `(key: string) -> string`  | Get environment variable |

Built with `mlua` 0.10, Lua 5.4 runtime. Disable with `--no-default-features` on flexfetch-core.

## Templates

Default template uses [Tera](https://tera.netlify.app/) (Jinja2-like) syntax. Variables are set per module name.

Custom templates: place `.tera` files in `~/.config/flexfetch/templates/` (configurable path — WIP).

## Output Formats

- **text** — Renders selected modules through Tera template
- **json** — Machine-readable JSON with all module data

```json
{
  "os": { "id": "cachyos", "name": "CachyOS Linux", "arch": "x86_64" },
  "host": "cachyos-x8664",
  "uptime": "5h 49m"
}
```

## Project Structure

```
flexfetch/
├── Cargo.toml              # Workspace manifest
├── flexfetch-core/          # Detection library
│   └── src/
│       ├── lib.rs          # Crate root + re-exports
│       ├── module.rs       # Module trait, InfoValue, SystemInfo
│       ├── module_registry.rs  # Module registration + parallel dispatch
│       ├── config.rs       # Config loader (TOML)
│       ├── context.rs      # Runtime context (dirs, cache)
│       ├── template.rs     # Tera template engine
│       ├── cache.rs        # File-backed JSON cache with TTL
│       ├── error.rs        # Error types
│       └── modules/        # Individual detection modules
├── flexfetch-cli/           # CLI binary
│   └── src/main.rs
├── flexfetch-lua/           # Lua plugin host (optional)
│   └── src/lib.rs
└── templates/
    └── default.tera        # Default output template
```

## License

MIT

## Credits

- [neofetch](https://github.com/dylanaraps/neofetch) — the project that inspired flexfetch
- [fastfetch](https://github.com/fastfetch-cli/fastfetch) — similar tool, also Rust-based
- [Tera](https://tera.netlify.app/) — template engine
- [mlua](https://github.com/khvzak/mlua) — Lua bindings for Rust
- [clap](https://clap.rs/) — CLI argument parser
