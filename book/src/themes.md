# Themes

flexfetch ships 5+ curated theme presets with the same output, dramatically
different looks. Switch at runtime:

```bash
flexfetch --theme nord
flexfetch --theme tokyo-night
flexfetch --theme gruvbox
flexfetch --theme dracula
flexfetch --theme catppuccin
```

Override any preset with named colors in config:

```toml
[display]
theme = "catppuccin"
color_keys = "yellow"
color_values = "green"
color_sep = "red"
```

Colors resolve from a named set (`black`/`red`/`green`/`yellow`/`blue`/
`magenta`/`cyan`/`white` + `bright_*` + `bold`) or raw ANSI escapes
(`"\u001b[92m"`).

| Theme         | Keys   | Values |
| ------------- | ------ | ------ |
| `catppuccin`  | pink   | cyan   |
| `dracula`     | pink   | cyan   |
| `nord`        | blue   | green  |
| `gruvbox`     | yellow | green  |
| `tokyo-night` | blue   | cyan   |
| `none`        | —      | —      |

Titles support per-character gradients when `gradient_title = true`.
