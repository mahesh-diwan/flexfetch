# Plugins

Write custom info modules in Lua 5.4. No compilation, no Bash scripting.

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

## Plugin API

| Function               | Returns | Description              |
| ---------------------- | ------- | ------------------------ |
| `ctx.read_file(path)`  | string  | Read file contents       |
| `ctx.run_command(cmd)` | string  | Execute shell command    |
| `ctx.get_env(key)`     | string  | Get environment variable |

List plugins: `flexfetch --list-plugins`.

> Prebuilt binaries from Releases/install.sh exclude Lua (pure-Rust binary).
> Source builds (`cargo install --git`) include it by default.

Built with [mlua](https://github.com/khvzak/mlua) 0.10 (Lua 5.4).
