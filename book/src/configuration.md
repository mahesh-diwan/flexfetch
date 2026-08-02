# Configuration

Config lives at `~/.config/flexfetch/config.toml`. Generate a starter with:

```bash
flexfetch --gen-config
```

```toml
modules = ["title", "separator", "os", "host", "kernel", "uptime",
           "shell", "cpu", "memory", "colors"]

[display]
separator = ": "
key_width = 8
theme = "catppuccin"
box_style = "rounded"    # rounded | double | dotted | thick | ascii
frame = "none"           # none | single | double
progress_bars = true
gradient_title = true
palette_style = "blocks" # blocks | squares | dots

[cache]
ttl = 60                 # seconds, 0 to disable
```

## Custom modules (no code)

Define inline info sources that run a shell command on every fetch:

```toml
[custom]
my_temp = { command = "sensors | grep temp1", label = "Temp" }
sys_load = { command = "uptime | awk -F'load average:' '{print $2}'", label = "Load" }
```

## Cache

Cache is a JSON file at `~/.cache/flexfetch/`. Reduces repeated disk reads.
TTL = 60s by default; `0` disables caching.

## Presets

Built-in presets: `default`, `minimal`, `full`, `dev`, `server`, `laptop`,
`ci`, `neofetch`. User presets live in `~/.config/flexfetch/presets/<name>.toml`
as a `modules = [...]` array:

```bash
flexfetch --preset server
flexfetch --list-presets
```

## Hot reload

`--watch` and `--live` detect changes to the config file (by mtime) and
re-apply them on the next refresh — no restart needed.
