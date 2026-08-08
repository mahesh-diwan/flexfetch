# Quick Start

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

## Next steps

1. **Customize** — run `flexfetch --wizard` for an interactive config builder
2. **Pick a theme** — `flexfetch --list-themes` shows all 28 presets
3. **Select modules** — `flexfetch --list-modules` shows all 39 built-in modules
4. **Write a plugin** — drop a `.lua` file in `~/.config/flexfetch/plugins/`
5. **Try the dashboard** — `flexfetch --live` for real-time monitoring

For the full flag list see the [CLI reference](output.md) or `flexfetch --help`.
