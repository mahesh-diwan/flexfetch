# Output & Export

## Formats

| Format     | Use case                     |
| ---------- | ---------------------------- |
| `text`     | Terminal (default)           |
| `json`     | Scripts, tooling             |
| `markdown` | Documentation, GitHub README |
| `svg`      | Vector graphics              |
| `html`     | Web embedding                |
| `png`      | Screenshots                  |

JSON mode disables ASCII art and themes. Output is structured for parsing:

```bash
flexfetch -f json | jq '.os.name'
flexfetch -f markdown > system-info.md
flexfetch --export svg --output out.svg
```

## Full CLI reference

```
Usage: flexfetch [OPTIONS]

Options:
  -c, --config <CONFIG>          Config file path
  -m, --modules <MODULES>        Colon-separated module list ("os:kernel:uptime")
  -t, --template <TEMPLATE>      Template name
  -f, --format <FORMAT>          Output format [default: text] (text|json|markdown)
      --theme <THEME>            Theme preset
      --debug                    Debug output
      --gen-config               Generate default config
      --list-modules             List built-in modules
      --list-presets             List presets
      --benchmark [N]            Per-module timing, or N full-pipeline runs
      --pipe                     Force pipe mode (no colors)
      --minimal | --full | --dev Module group presets
      --preset <NAME>            User/built-in preset
      --export <FMT>             Export to svg|html|png|markdown
  -o, --output <FILE>            Export output path
      --no-gradient | --no-progress
      --box-style <STYLE>        rounded|double|dotted|thick|ascii
      --palette-style <STYLE>    blocks|squares|dots
      --frame <STYLE>            none|single|double
      --watch [--watch-interval N]
      --live                     Real-time dashboard
      --smart                    Add git/project/context modules
      --health                   Add health score module
      --prompt                   Single-line prompt string
      --motd                     Plain-text banner (ANSI stripped)
      --ssh <HOST>               Remote fetch (repeatable)
      --wizard                   Interactive config wizard
  -h, --help
  -V, --version                  Show version + compiled features

Subcommands (default builds):
  completions <shell>            Generate shell completions (bash|zsh|fish|...)
```
