# Themes

flexfetch ships 27 curated theme presets with the same output, dramatically
different looks. Switch at runtime:

```bash
flexfetch --theme nord
flexfetch --theme tokyo-night
flexfetch --theme gruvbox
flexfetch --theme dracula
flexfetch --theme catppuccin
```

`flexfetch --list-themes` prints every preset name.

## Built-in themes

Keys and values are the 16-color ANSI codes each preset maps to its key/value
slots (the terminal also gets per-slot truecolor RGB when it supports 24-bit
color).

| Theme                  | Keys             | Values            |
| ---------------------- | ---------------- | ----------------- |
| `catppuccin`           | bright-blue      | bright-cyan       |
| `catppuccin-mocha`     | blue             | cyan              |
| `catppuccin-frappe`    | blue             | cyan              |
| `catppuccin-macchiato` | blue             | cyan              |
| `dracula`              | bright-magenta   | bright-cyan       |
| `nord`                 | bright-blue      | bright-green      |
| `gruvbox`              | bright-yellow    | bright-green      |
| `tokyo-night`          | bright-blue      | bright-cyan       |
| `tokyo-night-storm`    | blue             | cyan              |
| `solarized-dark`       | cyan             | blue              |
| `solarized-light`      | blue             | cyan              |
| `rose-pine`            | cyan             | magenta           |
| `rose-pine-dawn`       | cyan             | magenta           |
| `everforest-dark`      | blue             | cyan              |
| `everforest-light`     | blue             | cyan              |
| `bamboo`               | green            | cyan              |
| `oxocarbon-dark`       | cyan             | magenta           |
| `one-dark`             | red              | green             |
| `one-light`            | red              | green             |
| `monokai`              | bold bright-green| bright-red        |
| `monokai-pro`          | bold bright-cyan | bright-yellow     |
| `ayu-dark`             | bold bright-cyan | bright-green      |
| `ayu-mirage`           | bold bright-cyan | bright-yellow     |
| `palenight`            | bold bright-cyan | bright-green      |
| `material-ocean`       | bold bright-cyan | bright-red        |
| `kanagawa`             | bold bright-cyan | bright-green      |
| `mellow-purple`        | bold bright-cyan | bright-green      |

## Custom colors

Override any preset with named colors in config:

```toml
[display]
theme = "catppuccin"
color_keys = "yellow"
color_values = "green"
color_sep = "red"
```

Colors resolve from a named set:

| Color      | ANSI code |
| ---------- | --------- |
| `black`    | 30        |
| `red`      | 31        |
| `green`    | 32        |
| `yellow`   | 33        |
| `blue`     | 34        |
| `magenta`  | 35        |
| `cyan`     | 36        |
| `white`    | 37        |
| `bright_*` | 90-97     |
| `bold`     | 1         |

Or use raw ANSI escapes: `"\u001b[92m"`.

## Title gradients

When `gradient_title = true` (default), titles render with a per-character
gradient using the theme's gradient colors. Disable with:

```toml
[display]
gradient_title = false
```

## Random theme

```bash
flexfetch --theme random
```

Picks a random preset on each run.
