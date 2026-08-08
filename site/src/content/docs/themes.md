---
title: Themes
description: 28 built-in theme presets, custom colors, and title gradients
order: 7
---

flexfetch ships 28 curated theme presets with the same output, dramatically
different looks. Switch at runtime:

```bash
flexfetch --theme nord
flexfetch --theme tokyo-night
flexfetch --theme gruvbox
flexfetch --theme dracula
flexfetch --theme catppuccin
```

## Built-in themes

| Theme                  | Keys   | Values | Description              |
| ---------------------- | ------ | ------ | ------------------------ |
| `catppuccin`           | blue   | teal   | Catppuccin Mocha         |
| `catppuccin-mocha`     | blue   | teal   | Catppuccin Mocha (alias) |
| `catppuccin-frappe`    | blue   | teal   | Catppuccin Frappe        |
| `catppuccin-macchiato` | blue   | teal   | Catppuccin Macchiato     |
| `dracula`              | purple | cyan   | Dracula                  |
| `nord`                 | blue   | green  | Nord                     |
| `gruvbox`              | yellow | green  | Gruvbox                  |
| `tokyo-night`          | blue   | cyan   | Tokyo Night              |
| `tokyo-night-storm`    | blue   | cyan   | Tokyo Night Storm        |
| `solarized-dark`       | blue   | green  | Solarized Dark           |
| `solarized-light`      | blue   | green  | Solarized Light          |
| `rose-pine`            | pink   | teal   | Rose Pine                |
| `rose-pine-dawn`       | pink   | teal   | Rose Pine Dawn           |
| `everforest-dark`      | yellow | green  | Everforest Dark          |
| `everforest-light`     | yellow | green  | Everforest Light         |
| `bamboo`               | yellow | green  | Bamboo                   |
| `oxocarbon-dark`       | blue   | cyan   | Oxocarbon Dark           |
| `one-dark`             | purple | cyan   | One Dark                 |
| `one-light`            | purple | cyan   | One Light                |
| `monokai`              | yellow | green  | Monokai                  |
| `monokai-pro`          | yellow | green  | Monokai Pro              |
| `ayu-dark`             | yellow | cyan   | Ayu Dark                 |
| `ayu-mirage`           | yellow | cyan   | Ayu Mirage               |
| `palenight`            | purple | cyan   | Material Palenight       |
| `material-ocean`       | purple | cyan   | Material Ocean           |
| `kanagawa`             | red    | teal   | Kanagawa                 |
| `mellow-purple`        | purple | green  | Mellow Purple            |
| `none`                 | —      | —      | No colors                |

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