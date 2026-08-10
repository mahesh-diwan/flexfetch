# Output & Export

## Formats

| Format     | Use case                     | Command                 |
| ---------- | ---------------------------- | ----------------------- |
| `text`     | Terminal (default)           | `flexfetch`             |
| `json`     | Scripts, tooling             | `flexfetch -f json`     |
| `markdown` | Documentation, GitHub README | `flexfetch -f markdown` |

JSON mode disables ASCII art and themes. Output is structured for parsing:

```bash
flexfetch -f json | jq '.os.name'
flexfetch -f markdown > system-info.md
```

## Export

Export to file-based formats:

```bash
flexfetch --export svg --output out.svg
flexfetch --export html --output out.html
flexfetch --export png --output out.png
flexfetch --export markdown --output out.md
```

SVG and HTML exports parse the ANSI-colored terminal output and render it as
styled vector/web graphics. PNG export requires the `image-logos` feature.

## Full CLI reference

```
Usage: flexfetch [OPTIONS] [COMMAND]

Commands:
  completions  Generate shell completions for the given shell
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>          Config file path
  -m, --modules <MODULES>        Colon-separated module list ("os:kernel:uptime")
  -t, --template <TEMPLATE>      Template name
  -f, --format <FORMAT>          Output format [default: text] (text|json|markdown)
      --theme <THEME>            Theme preset name (or "random")
      --debug                    Debug output
      --gen-config               Generate default config
      --list-modules             List built-in modules
      --list-presets             List presets
      --list-themes              List all theme presets
      --benchmark [N]            Per-module timing, or N full-pipeline runs
      --pipe                     Force pipe mode (no colors)
      --minimal                  Minimal module group
      --flash                    Fastest possible output (no config/theme)
      --full                     Full module group
      --dev                      Dev module group
      --preset <NAME>            User/built-in preset
      --export <FMT>             Export to svg|html|png|markdown
  -o, --output <FILE>            Export output path
      --no-gradient              Disable key gradient
      --no-progress              Disable progress bars
      --box-style <STYLE>        rounded|double|dotted|thick|ascii
      --palette-style <STYLE>    blocks|squares|dots
      --frame <STYLE>            none|single|double
      --watch                    Watch mode (refresh periodically)
      --watch-interval <SECS>    Watch interval in seconds [default: 2]
      --live                     Real-time dashboard
      --smart                    Add git/project/context modules
      --health                   Add health score module
      --prompt                   Single-line prompt string
      --motd                     Plain-text banner (ANSI stripped)
      --ssh <HOST>               Remote fetch (repeatable)
      --diff <A> <B>             Diff mode (local|host@remote|file)
      --wizard                   Interactive config wizard
      --qr                       Render config as QR code
      --import-qr <FILE>         Import config from QR code image
      --update                   Self-update from GitHub releases
      --doctor                   Environment diagnostics
      --hook <SHELL>             Print shell hook (bash|zsh|fish)
      --update-db                Refresh crowdsourced hardware database
      --auto-theme               Derive theme from wallpaper colors
      --demo                     Showcase mode (all modules + features)
      --bug-report               Print environment/version dump for bug reports
  -h, --help                     Print help
  -V, --version                  Show version + compiled features
```

## Benchmark

Per-module timing or full-pipeline micro-benchmark:

```bash
flexfetch --benchmark           # per-module timing
flexfetch --benchmark 10        # run pipeline 10 times, report min/avg/total
```
