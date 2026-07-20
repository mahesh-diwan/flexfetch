# flexfetch

Fast, flexible system information tool for Linux and macOS.

## Quick start

```bash
cargo install --path sysfetch-cli
flexfetch
```

## Configuration

Config at `~/.config/flexfetch/config.toml`:

```toml
modules = ["os", "host", "kernel", "uptime", "cpu", "memory", "disk", "colors"]

[display]
separator = ": "
key_width = 8
```

## Plugins

Write Lua plugins in `~/.config/flexfetch/plugins/`:

```lua
return {
    name = "my_plugin",
    collect = function(ctx)
        return { value = ctx.run_command("echo hello"), type = "scalar" }
    end
}
```

## CLI

```
flexfetch [-c config] [-m os:cpu:memory] [-f json] [--debug]
```

## License

MIT
