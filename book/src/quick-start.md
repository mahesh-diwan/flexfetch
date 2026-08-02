# Quick Start

Run it with no arguments to fetch the default module set:

```bash
flexfetch
```

Common commands:

| Command                              | What it does                                  |
| ------------------------------------ | --------------------------------------------- |
| `flexfetch --theme nord`             | Switch theme                                  |
| `flexfetch -m "os:kernel:uptime"`    | Select modules explicitly                     |
| `flexfetch --preset minimal`         | Use a preset module list                      |
| `flexfetch -f json`                  | Structured JSON output                        |
| `flexfetch --list-modules`           | List all built-in modules                     |
| `flexfetch --gen-config`             | Generate `~/.config/flexfetch/config.toml`    |
| `flexfetch --live`                   | Real-time dashboard                           |
| `flexfetch --wizard`                 | Interactive config wizard                     |
| `flexfetch --prompt`                 | One-line shell prompt string                  |
| `flexfetch --ssh host`               | Fetch a remote host's info                    |
| `flexfetch completions bash`         | Print bash completions                        |

For the full flag list see the [CLI reference](output.md) or `flexfetch --help`.
