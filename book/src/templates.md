# Templates

flexfetch renders output through [Tera](https://tera.netlify.app/) (Jinja2/Django
syntax) templates. The default template renders side-by-side logo + info with
fastfetch-style tree connectors.

## Context variables

### Module values

Each module injects its data as a Tera variable:

- **Scalars:** `kernel`, `host`, `uptime`, `shell`, `de`, `cpuusage`, `processes`
- **Maps:** `os` (with `name`, `id`, `version`, `pretty_name`), `locale`, `shell`, `cpu`, `memory`, `gpu`, `disk`, `network`, `battery`, `temperature`, `wifi`, `publicip`, `dns`, `media`, `bluetooth`, `display`, `resolution`, `colors`, `packages`, `terminal`, `wm`, `git`, `project`, `context`, `health`, `wallpaper`, `weather`, `container`, `fsdeep`

### Theme variables

| Variable         | Description                          |
| ---------------- | ------------------------------------ |
| `theme_title`    | Title color escape sequence          |
| `theme_keys`     | Key color escape sequence            |
| `theme_values`   | Value color escape sequence          |
| `theme_sep`      | Separator color escape sequence      |
| `theme_section`  | Section header color escape sequence |
| `theme_reset`    | Reset escape sequence                |
| `theme_gradient` | Gradient color array for titles      |

### Display variables

| Variable                | Description                        |
| ----------------------- | ---------------------------------- |
| `display_separator`     | Key-value separator string         |
| `display_key_width`     | Minimum key column width           |
| `display_palette_style` | Color palette rendering style      |
| `display_progress_bars` | Whether to show progress bars      |
| `display_logo_gradient` | Whether to gradient the ASCII logo |
| `display_sections`      | Whether to show section headers    |

### Box variables

| Variable          | Description                |
| ----------------- | -------------------------- |
| `box_header_left` | Box header left character  |
| `box_header_line` | Box header horizontal line |
| `box_row`         | Box row character          |
| `box_sep`         | Box separator character    |

### Icons

Nerd Font icons for each module: `icon_os`, `icon_kernel`, `icon_cpu`,
`icon_memory`, `icon_disk`, etc. These are blanked when Nerd Font detection
fails (no tofu boxes).

### Image logos

| Variable            | Description                     |
| ------------------- | ------------------------------- |
| `image_logos`       | Per-module image logo paths     |
| `distro_image_logo` | Distro-specific image logo path |

## Custom templates

Place templates in `~/.config/flexfetch/templates/`:

```bash
flexfetch -t my_template
```

Default template path: `~/.config/flexfetch/templates/default.tera`.

## Tera filters

The template engine registers these custom filters:

| Filter            | Usage                                                  |
| ----------------- | ------------------------------------------------------ |
| `palette_display` | Render color palette (`blocks`/`squares`/`dots` style) |
| `progress_bar`    | Render a progress bar from a percentage                |
| `pad`             | Pad string to fixed visible width                      |
| `osc8`            | Wrap text in an OSC-8 hyperlink                        |

Example usage in a template:

```tera
{{ os.pretty_name | pad(width=12) }}
{{ disk.usage | progress_bar(width=20) }}
{{ wifi.ssid | osc8(url="https://example.com") }}
```

## The plain fallback

The minimal build (`--no-default-features`) drops the tera engine to save
~4 MB. It uses a built-in plain renderer that emits fastfetch-style
`├─ Key: value` lines with the same labels the default template uses — readable
output, no template files.

> In minimal builds, a `template` config value is ignored (there is no
> template engine to honor it). SVG and HTML exports still work (they render
> from the plain renderer); PNG export requires the `image-logos` feature.
