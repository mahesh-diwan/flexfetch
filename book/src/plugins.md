# Plugins

Write custom info modules in Lua 5.4 or sandboxed WebAssembly. No compilation,
no Bash scripting.

## Lua plugins

Drop a `.lua` file in `~/.config/flexfetch/plugins/`:

```lua
-- ~/.config/flexfetch/plugins/user.lua
return {
    name = "user",
    collect = function(ctx)
        local user = ctx.get_env("USER")
        local shell = ctx.get_env("SHELL")
        return { value = user .. " (" .. shell .. ")" }
    end
}
```

The plugin appears in output automatically. Use it in your module list:

```toml
modules = ["os", "kernel", "user"]
```

### Plugin structure

A Lua plugin returns a table with:

| Field     | Type       | Description                           |
| --------- | ---------- | ------------------------------------- |
| `name`    | `string`   | Module name (must be unique)          |
| `collect` | `function` | Called to collect info, returns table |

### Return types

**Scalar** — a single string value:

```lua
return { value = "hello" }
```

**Map** — key-value pairs:

```lua
return { model = "i5-12450H", cores = "8" }
```

**List** — array of strings:

```lua
return { "item1", "item2", "item3" }
```

**Table** — array of row maps:

```lua
return {
    { label = "Row 1", value = "val1" },
    { label = "Row 2", value = "val2" },
}
```

### Plugin API

| Function               | Returns | Description              |
| ---------------------- | ------- | ------------------------ |
| `ctx.read_file(path)`  | string  | Read file contents       |
| `ctx.run_command(cmd)` | string  | Execute shell command    |
| `ctx.get_env(key)`     | string  | Get environment variable |

### Managing plugins

```bash
flexfetch --list-plugins        # list installed plugins
flexfetch plugin search <query> # search the hosted registry
flexfetch plugin install <name> # install from registry
flexfetch plugin update         # update all installed plugins
```

> Prebuilt binaries from Releases/install.sh exclude Lua (pure-Rust binary).
> Source builds (`cargo install --git`) include it by default.

Built with [mlua](https://github.com/khvzak/mlua) 0.10 (Lua 5.4).

## WASM plugins

Sandboxed WebAssembly plugins with capability-gated host imports:

```rust
// Plugin exports flexfetch_plugin() -> i64 (packed ptr+len)
// Host provides: log, env_get (Env cap), read_file (File cap), run_command (Command cap)
```

### Sandboxing

| Resource     | Limit            |
| ------------ | ---------------- |
| Instructions | 10M (fuel)       |
| Memory       | 64 MB            |
| Host calls   | Capability-gated |

Available capabilities:

- `log` — always available
- `env_get` — requires Env capability
- `read_file` — requires File capability
- `run_command` — requires Command capability

WASM plugins are opt-in at build time:

```bash
cargo build --release --features wasm-plugins
```

## Plugin registry

The hosted registry at GitHub provides verified plugins:

```bash
flexfetch plugin search "system info"
flexfetch plugin install <name>
```

Plugins are verified with SHA-256 checksums and Ed25519 signatures.
