# Flexfetch — System Info Tool Design

**Date:** 2026-07-20
**Status:** Draft

## Goal

Build fast, extensible, modular system information CLI tool in Rust. Differentiate from fastfetch/macchina via Lua plugin system, Tera template engine, and minimal binary size.

## Naming

**flexfetch** — "flexible fetch." Communicates extensibility and modularity in one word.

## Architecture

Workspace of 3 crates:

```
flexfetch/
├── Cargo.toml              # workspace root
├── sysfetch-core/          # library: detection, plugin engine, template engine
├── sysfetch-cli/           # CLI binary: config, flags, orchestration
└── sysfetch-lua/           # Lua bindings for plugin API
```

### Data Flow

1. CLI parses args -> loads TOML config
2. Config defines module list, template path, plugin paths
3. Module registry resolves built-in + Lua modules
4. Rayon thread pool executes modules in parallel
5. Each module returns `InfoValue` (Scalar, Map, List, Table)
6. SystemInfo struct collected (serde-serializable to JSON)
7. Tera template renders output to stdout

### Module Trait

```rust
pub trait Module: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self, ctx: &Context) -> Result<InfoValue>;
}
```

Lua modules implement same trait via adapter in sysfetch-lua.

### Built-in Modules

| Module    | Data                          | Source                         |
| --------- | ----------------------------- | ------------------------------ |
| os        | name, version, arch, build ID | /etc/os-release, uname         |
| host      | hostname                      | gethostname                    |
| kernel    | version, release              | uname                          |
| cpu       | model, cores, freq, cache     | /proc/cpuinfo, sysctl          |
| memory    | total, used, free, swap, %    | /proc/meminfo, vm_stat         |
| disk      | mounts, usage, fs type        | statvfs, getfsstat             |
| gpu       | vendor, model, driver         | lspci / IOKit                  |
| packages  | count per package manager     | dpkg, pacman, rpm, flatpak DBs |
| shell     | name, version                 | $SHELL                         |
| terminal  | emulator                      | $TERM_PROGRAM                  |
| de        | desktop environment           | XDG_CURRENT_DESKTOP            |
| wm        | window manager                | wmctrl / $XDG_SESSION_TYPE     |
| uptime    | human-readable                | /proc/uptime                   |
| network   | interfaces, IPs               | /sys/class/net, getifaddrs     |
| battery   | %, status, time               | /sys/class/power_supply, IOKit |
| processes | running process count         | /proc/stat, proc_list          |
| locale    | LANG, encoding                | locale                         |
| colors    | 8-color palette swatches      | ANSI escapes                   |
| custom    | user-defined shell commands   | config                         |
| lua       | user-defined Lua scripts      | .lua files in plugin dir       |

### Lua Plugin API

```lua
-- Plugin returns table with name + collect function
return {
    name = "docker_containers",
    collect = function(ctx)
        local output = ctx:run_command("docker ps -q 2>/dev/null | wc -l")
        if output == "" then return nil end
        return { value = output .. " containers", type = "scalar" }
    end
}
```

API surface: `ctx:read_file(path)`, `ctx:run_command(cmd)`, `ctx:get_env(key)`, `ctx:cache(key, ttl_secs)`.

Lua sandboxed via mlua pcall — plugin crash doesn't crash binary.

### Template Engine

Tera templates in `~/.config/flexfetch/templates/`. Default layout:

```tera
{{ logo(style="auto") }}
{{ key_value("OS", os.name ~ " " ~ os.version) }}
{{ key_value("Kernel", kernel.version) }}
{{ key_value("CPU", cpu.model ~ " (" ~ cpu.cores ~ " cores)") }}
{{ key_value("Memory", memory.used ~ " / " ~ memory.total ~ " (" ~ memory.percent ~ "%)") }}
{{ key_value("Disk", disk.mounts | map(attr="usage") | join(", ")) }}
{{ colors_swatches() }}
```

Built-in blocks: `logo()`, `key_value()`, `progress_bar()`, `colors_swatches()`, `separator()`, `title()`.

### CLI Interface

```
flexfetch                         # default config
flexfetch -c custom.toml          # custom config
flexfetch -m cpu:memory:disk      # override modules
flexfetch -t my-template.tera     # custom template
flexfetch -f json                 # JSON output
flexfetch --gen-config            # write default config.toml
flexfetch --list-modules          # list available modules
flexfetch --list-plugins          # list found Lua plugins
flexfetch --debug                 # verbose per-module errors
```

### Configuration (TOML)

```toml
modules = ["os", "host", "kernel", "cpu", "memory", "disk", "packages"]
plugins_dir = "~/.config/flexfetch/plugins"
template = "default.tera"

[cache]
ttl = 60  # seconds

[display]
separator = ": "
key_width = 8
color_keys = "cyan"
color_values = "reset"

[templates]
main = "default.tera"
```

### Error Handling

- Module failure produces `null` in SystemInfo struct
- Template renders `N/A` for missing keys (Tera default)
- `--debug` shows per-module error details
- Lua plugins wrapped in pcall

### Caching

- `/tmp/flexfetch-cache.json` for expensive detections
- TTL configurable (default 60s)
- Applies to: package counts, GPU detection, network interfaces
- Skipped on `--no-cache`

### Testing

- Unit tests per module with fixture files
- Integration tests: `flexfetch -f json` -> validate JSON structure
- Lua plugin tests: run sample scripts, check output shape
- Benchmarks via criterion vs fastfetch

### Binary Size & Speed Targets

- **Binary:** <2MB stripped (LTO + strip)
- **Startup:** <5ms cold, <2ms warm
- **Dependencies:** minimal — avoid heavy crates where possible

## Key Decisions

| Decision           | Choice        | Rationale                           |
| ------------------ | ------------- | ----------------------------------- |
| Language           | Rust          | Speed, safety, ecosystem            |
| Config format      | TOML          | Clean, Rust-native (serde)          |
| Template engine    | Tera          | Battle-tested, safe, Rust-native    |
| Plugin language    | Lua (mlua)    | Lightweight, embeddable, well-known |
| Parallel execution | rayon         | De facto Rust parallel standard     |
| Distribution       | Single binary | Static link Lua, no runtime dep     |
