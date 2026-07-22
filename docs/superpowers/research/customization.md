# Customization Research

## Current State

flexfetch has a basic config system:

- TOML config at `~/.config/flexfetch/config.toml`
- CLI overrides: `--theme`, `--format`, `--modules`, `--config`
- Per-field color overrides at global level only (no per-module)
- No config cascade, no presets, no project-level config

## Config Cascade (fastfetch pattern)

fastfetch uses a 4-level cascade:

1. **Built-in defaults** (compiled into binary)
2. **User config** (`~/.config/fastfetch/config.jsonc`)
3. **Project config** (`./.fastfetch.jsonc` in CWD)
4. **CLI flags** (`--key value`)

Priority: CLI > project > user > defaults.

**Recommendation for flexfetch:**

1. Built-in defaults (compiled)
2. User config (`$XDG_CONFIG_HOME/flexfetch/config.toml`)
3. Project config (`./flexfetch.toml` in CWD)
4. CLI flags

Merge with deep override: only specified fields override, rest inherit.

## Per-Module Overrides

fastfetch allows per-module color/format:

```jsonc
{
  "cpu": {
    "color": "blue",
    "keyIcon": "CPU:",
  },
  "memory": {
    "color": "green",
  },
}
```

**Recommendation for flexfetch:**

```toml
[modules.cpu]
color_keys = "\u001b[34m"     # blue
color_values = "\u001b[97m"   # white

[modules.memory]
color_keys = "\u001b[32m"     # green
```

Per-module overrides override global theme for that module only.

## Preset System

A preset is a named bundle of: theme + modules + logo style.

```toml
# ~/.config/flexfetch/presets/dev.toml
theme = "catppuccin"
modules = ["os", "kernel", "cpu", "memory", "disk", "git-branch"]
logo = "arch"

# ~/.config/flexfetch/presets/minimal.toml
theme = "dracula"
modules = ["os", "kernel", "uptime"]
logo = "generic"
```

Usage: `flexfetch --preset dev`

**Built-in presets:** default, minimal, dev, server, laptop (with battery).

## Config File Format

TOML is the right choice:

- Already used by Rust ecosystem (Cargo.toml)
- Human-readable, writable
- Supports nested tables for per-module overrides
- No need for JSONC complexity

## Migration Path

Current config remains compatible. New fields are additive with `#[serde(default)]`:

- `[[presets]]` section for named presets
- `[modules.<name>]` for per-module overrides
- `logo` field for logo selection

No breaking changes.
