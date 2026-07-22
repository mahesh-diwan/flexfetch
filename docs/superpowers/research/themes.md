# Theme Systems Research

## Current flexfetch Theme Structure

flexfetch uses 4 color slots per theme: `title`, `keys`, `values`, `sep` (separator). Each stores an ANSI escape code string. The `resolve()` function in `theme.rs` matches a preset name to its `Theme` const, then applies per-field config overrides (`color_title`, `color_keys`, `color_values`, `color_sep`).

Existing presets (5): catppuccin, dracula, nord, gruvbox, tokyo-night.

---

## Tool Comparison

### fastfetch

- **Config format**: JSONC (`~/.config/fastfetch/config.jsonc`)
- **Color slots**: `keys`, `title`, `separator` (global), plus per-module `keyColor` and `outputColor`
- **Color format**: Named colors (`"blue"`, `"red"`), not ANSI codes — fastfetch resolves internally
- **Presets**: ~30 built-in examples (all.jsonc, neofetch.jsonc, paleofetch.jsonc, screenfetch.jsonc, archey.jsonc, plus 25+ numbered examples)
- **Key difference from flexfetch**: Per-module color overrides are first-class. Each module can have its own `keyColor`. No "values" slot — output color is separate from key color.

### neofetch

- **Config format**: Bash script (`~/.config/neofetch/config`)
- **Color system**: 6-element array: `colors=(title @ underline subtitle colon info)`
- **Values**: Color numbers 0-7 (normal) and 8-15 (bright), or `"distro"` for auto-coloring
- **Auto-coloring**: `distro` picks colors from a built-in per-distro palette
- **Key difference**: Underline and subtitle slots. Auto-coloring based on distro logo. Bash-based config (not portable).

### macchina

- **Config format**: TOML theme files (`~/.config/macchina/themes/`)
- **Color slots**: `key_color`, `separator_color` (global), per-key overrides in `[keys]` section
- **Color format**: Named colors (`"Cyan"`, `"White"`), hex (`"#00FF00"`), or indexed (`"046"`)
- **Key difference**: Per-key color overrides via `[keys]` section. Supports randomization of key/separator colors. Clean TOML format.

---

## Recommended New Presets

### 1. Solarized Dark

| Slot   | Color  | ANSI Code                  |
| ------ | ------ | -------------------------- |
| title  | yellow | `\x1b[1;33m` (bold yellow) |
| keys   | cyan   | `\x1b[36m`                 |
| values | blue   | `\x1b[34m`                 |
| sep    | base01 | `\x1b[90m` (bright black)  |

### 2. Solarized Light

| Slot   | Color  | ANSI Code                                   |
| ------ | ------ | ------------------------------------------- |
| title  | orange | `\x1b[1;31m` (bold red — closest to orange) |
| keys   | blue   | `\x1b[34m`                                  |
| values | cyan   | `\x1b[36m`                                  |
| sep    | base01 | `\x1b[90m`                                  |

### 3. Rosé Pine

| Slot   | Color | ANSI Code                                     |
| ------ | ----- | --------------------------------------------- |
| title  | pine  | `\x1b[1;36m` (bright cyan)                    |
| keys   | foam  | `\x1b[36m` (cyan — #9ccfd8 → closest)         |
| values | iris  | `\x1b[35m` (magenta — #c4a7e7 → closest)      |
| sep    | muted | `\x1b[90m` (bright black — #6e6a86 → closest) |

### 4. Rosé Pine Dawn

| Slot   | Color | ANSI Code                           |
| ------ | ----- | ----------------------------------- |
| title  | pine  | `\x1b[1;34m` (bold blue — #286983)  |
| keys   | foam  | `\x1b[36m` (cyan — #56949F)         |
| values | iris  | `\x1b[35m` (magenta — #907AA9)      |
| sep    | muted | `\x1b[90m` (bright black — #9893A5) |

### 5. Everforest Dark

| Slot   | Color | ANSI Code                             |
| ------ | ----- | ------------------------------------- |
| title  | green | `\x1b[1;32m` (bright green — #a7c080) |
| keys   | blue  | `\x1b[34m` (blue — #7fbbb3 → closest) |
| values | aqua  | `\x1b[36m` (cyan — #83c092)           |
| sep    | grey0 | `\x1b[90m` (bright black — #7a8478)   |

### 6. Everforest Light

| Slot   | Color | ANSI Code                           |
| ------ | ----- | ----------------------------------- |
| title  | green | `\x1b[1;32m` (green — #8da101)      |
| keys   | blue  | `\x1b[34m` (blue — #3a94c5)         |
| values | aqua  | `\x1b[36m` (cyan — #35a77c)         |
| sep    | grey0 | `\x1b[90m` (bright black — #a6b0a0) |

### 7. Bamboo

| Slot   | Color     | ANSI Code                                   |
| ------ | --------- | ------------------------------------------- |
| title  | vermilion | `\x1b[1;31m` (bold red — #E06848 → closest) |
| keys   | sage      | `\x1b[32m` (green — #68A870)                |
| values | porcelain | `\x1b[36m` (cyan — #5898A8)                 |
| sep    | muted     | `\x1b[90m` (bright black — #888A90)         |

### 8. Oxocarbon Dark

| Slot   | Color   | ANSI Code                            |
| ------ | ------- | ------------------------------------ |
| title  | blue    | `\x1b[1;36m` (bright cyan — #08bdba) |
| keys   | cyan    | `\x1b[36m` (cyan — #33b1ff)          |
| values | magenta | `\x1b[35m` (magenta — #ff7eb6)       |
| sep    | grey    | `\x1b[90m` (bright black — #525252)  |

### 9. One Dark

| Slot   | Color  | ANSI Code                             |
| ------ | ------ | ------------------------------------- |
| title  | purple | `\x1b[1;35m` (bold magenta — #c678dd) |
| keys   | red    | `\x1b[31m` (red — #e06c75)            |
| values | green  | `\x1b[32m` (green — #98c379)          |
| sep    | grey   | `\x1b[90m` (bright black — #5c6370)   |

### 10. One Light

| Slot   | Color  | ANSI Code                             |
| ------ | ------ | ------------------------------------- |
| title  | purple | `\x1b[1;35m` (bold magenta — #a626a4) |
| keys   | red    | `\x1b[31m` (red — #e45649)            |
| values | green  | `\x1b[32m` (green — #50a14f)          |
| sep    | grey   | `\x1b[90m` (bright black — #a0a1a7)   |

### 11. Tokyo Night Storm (variant)

| Slot   | Color  | ANSI Code                             |
| ------ | ------ | ------------------------------------- |
| title  | purple | `\x1b[1;35m` (bold magenta — #bb9af7) |
| keys   | blue   | `\x1b[34m` (blue — #7aa2f7)           |
| values | cyan   | `\x1b[36m` (cyan — #7dcfff)           |
| sep    | grey   | `\x1b[90m` (bright black — #565f89)   |

### 12. Catppuccin Mocha

| Slot   | Color    | ANSI Code                             |
| ------ | -------- | ------------------------------------- |
| title  | mauve    | `\x1b[1;35m` (bold magenta — #cba6f7) |
| keys   | blue     | `\x1b[34m` (blue — #89b4fa)           |
| values | teal     | `\x1b[36m` (cyan — #94e2d5)           |
| sep    | overlay0 | `\x1b[90m` (bright black — #6c7086)   |

### 13. Catppuccin Frappé

| Slot   | Color    | ANSI Code                             |
| ------ | -------- | ------------------------------------- |
| title  | mauve    | `\x1b[1;35m` (bold magenta — #ca9ee6) |
| keys   | blue     | `\x1b[34m` (blue — #8caaee)           |
| values | teal     | `\x1b[36m` (cyan — #81c8be)           |
| sep    | overlay0 | `\x1b[90m` (bright black — #737978)   |

### 14. Catppuccin Macchiato

| Slot   | Color    | ANSI Code                             |
| ------ | -------- | ------------------------------------- |
| title  | mauve    | `\x1b[1;35m` (bold magenta — #c6a0f6) |
| keys   | blue     | `\x1b[34m` (blue — #8aadf4)           |
| values | teal     | `\x1b[36m` (cyan — #8bd5ca)           |
| sep    | overlay0 | `\x1b[90m` (bright black — #6e738d)   |

---

## Gradient/Rainbow Support

### Approach

Gradients require **truecolor (24-bit)** support. The ANSI escape sequence `\x1b[38;2;R;G;Bm` sets foreground to an exact RGB value.

**Technique**: For each character in the title string, interpolate between two (or more) RGB stop colors, then emit `\x1b[38;2;R;G;Bm<char>` for each character.

**Implementation sketch** (Rust):

```rust
fn gradient_char(ch: char, pos: usize, len: usize, start: [u8; 3], end: [u8; 3]) -> String {
    let t = pos as f64 / (len - 1).max(1) as f64;
    let r = (start[0] as f64 + t * (end[0] as f64 - start[0] as f64)) as u8;
    let g = (start[1] as f64 + t * (end[1] as f64 - start[1] as f64)) as u8;
    let b = (start[2] as f64 + t * (end[2] as f64 - start[2] as f64)) as u8;
    format!("\x1b[38;2;{};{};{}m{}", r, g, b, ch)
}

fn gradient_text(text: &str, start: [u8; 3], end: [u8; 3]) -> String {
    text.chars()
        .enumerate()
        .map(|(i, c)| gradient_char(c, i, text.len(), start, end))
        .collect::<String>()
        + "\x1b[0m"
}
```

**Config option**:

```toml
[display]
gradient = true
gradient_colors = ["#7aa2f7", "#bb9af7", "#f7768e"]  # rainbow stops
```

**Fallback**: If terminal doesn't support truecolor, detect via `$COLORTERM` or `$TERM` and fall back to the `title` color slot.

**Note**: The current 16-color ANSI palette doesn't support smooth gradients — only truecolor (256-color cube gives rough approximation). Always check terminal capability first.

---

## Per-Field Override Design

### Current State

flexfetch already supports per-field overrides via config:

```toml
[display]
color_title = "red"
color_keys = "blue"
color_values = "cyan"
color_sep = "bright-black"
```

### Recommended Enhancement

Add **per-module overrides** (like fastfetch's `keyColor` per module):

```toml
[display.theme]
preset = "catppuccin"

# Global overrides (applies to all modules)
color_title = "red"
color_keys = "blue"
color_values = "cyan"
color_sep = "bright-black"

# Per-module overrides (optional, takes precedence)
[[display.modules]]
type = "cpu"
key_color = "yellow"

[[display.modules]]
type = "memory"
key_color = "green"
```

### Implementation

1. Extend the template context to include per-module color values
2. In the template, use `{% if module_key_color %}{{ module_key_color }}{% else %}{{ keys }}{% endif %}`
3. Keep backward compatibility — global overrides still work

### Design Decisions

- **Keep current 4-slot global system** — it covers 90% of use cases
- **Add optional per-module `key_color`** — only for users who want different colors per line
- **Don't add per-module `value_color`** — values should be uniform for readability
- **Gradient on title only** — values/keys with gradients are visually noisy

---

## Summary

| Feature            | fastfetch                  | neofetch                                       | macchina                | flexfetch (current)          |
| ------------------ | -------------------------- | ---------------------------------------------- | ----------------------- | ---------------------------- |
| Config format      | JSONC                      | Bash                                           | TOML                    | TOML                         |
| Global color slots | 3 (keys, title, separator) | 6 (title, @, underline, subtitle, colon, info) | 2 (key, separator)      | 4 (title, keys, values, sep) |
| Per-module colors  | Yes (keyColor)             | No                                             | Yes (per-key in [keys]) | No                           |
| Auto-coloring      | No                         | Yes (distro-based)                             | No                      | No                           |
| Gradient support   | No                         | No                                             | No                      | No                           |
| Built-in themes    | ~30 presets                | Distro-based                                   | ~10 themes              | 5 presets                    |
| Color format       | Named                      | Numbers 0-15                                   | Named/hex/indexed       | Named + ANSI codes           |

### New Presets Found: 14

1. Solarized Dark
2. Solarized Light
3. Rosé Pine
4. Rosé Pine Dawn
5. Everforest Dark
6. Everforest Light
7. Bamboo
8. Oxocarbon Dark
9. One Dark
10. One Light
11. Tokyo Night Storm
12. Catppuccin Mocha
13. Catppuccin Frappé
14. Catppuccin Macchiato

Total presets after implementation: 5 (existing) + 14 (new) = **19 presets**.
