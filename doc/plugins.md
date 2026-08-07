# FlexFetch Plugin Development Guide

## Overview

FlexFetch supports Lua 5.4 plugins for custom system info modules. Plugins are simple Lua files that return a module definition.

## Quick Start

Create a plugin at `~/.config/flexfetch/plugins/myplugin.lua`:

```lua
return {
    name = "myplugin",
    collect = function(ctx)
        return { value = "Hello from Lua!" }
    end
}
```

Run `flexfetch --list-plugins` to verify it's detected.

## Plugin Structure

Each plugin is a Lua file returning a table with:

```lua
return {
    name = "module_name",      -- Required: unique identifier
    collect = function(ctx)    -- Required: collection function
        -- Return value
        return { value = "string" }
        -- Or for maps:
        return { key1 = "value1", key2 = "value2" }
    end
}
```

## Plugin API

The `ctx` object provides these functions:

| Function               | Returns | Description              |
| ---------------------- | ------- | ------------------------ |
| `ctx.read_file(path)`  | string  | Read file contents       |
| `ctx.run_command(cmd)` | string  | Execute shell command    |
| `ctx.get_env(key)`     | string  | Get environment variable |

## Examples

### System Info Plugin

```lua
return {
    name = "system",
    collect = function(ctx)
        local user = ctx.get_env("USER") or "unknown"
        local host = ctx.run_command("hostname")
        return { value = user .. "@" .. host }
    end
}
```

### Temperature Plugin

```lua
return {
    name = "temp",
    collect = function(ctx)
        local temp = ctx.run_command("sensors | grep temp1 | awk '{print $2}'")
        return { value = temp:gsub("%s+", "") }
    end
}
```

### GPU Info Plugin

```lua
return {
    name = "gpu",
    collect = function(ctx)
        local gpu = ctx.run_command("lspci | grep VGA | awk -F': ' '{print $2}'")
        return { value = gpu }
    end
}
```

### Multiple Values Plugin

```lua
return {
    name = "network",
    collect = function(ctx)
        local ip = ctx.run_command("hostname -I | awk '{print $1}'")
        local iface = ctx.run_command("ip route | grep default | awk '{print $5}'")
        return {
            ip = ip:gsub("%s+", ""),
            interface = iface:gsub("%s+", "")
        }
    end
}
```

## Configuration

Enable/disable plugins in `~/.config/flexfetch/config.toml`:

```toml
[plugins]
enabled = true
```

## Development Tips

1. Use `ctx.run_command()` for shell commands
2. Use `ctx.read_file()` for direct file access (faster)
3. Use `ctx.get_env()` for environment variables
4. Return strings for simple values, tables for structured data
5. Test with `flexfetch --list-plugins` and `flexfetch -m myplugin`

## Troubleshooting

- Check Lua syntax: `luac -p myplugin.lua`
- Check plugin detection: `flexfetch --list-plugins`
- Run with debug: `flexfetch --debug -m myplugin`
