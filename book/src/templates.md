# Templates

flexfetch renders output through [Tera](https://tera.netlify.app/) (Jinja2/Django
syntax) templates. The default template renders side-by-side logo + info with
fastfetch-style tree connectors.

## Context variables

- **Scalars:** `kernel`, `host`, `uptime`
- **Maps:** `os.pretty_name`, `locale.lang`, `shell.name`, `cpu.model`,
  `memory.used`
- **Theme:** `theme_keys`, `theme_values`, `theme_reset`, `theme_title`,
  `theme_sep`, `theme_gradient`
- **Display:** `display_separator`, `display_key_width`
- **Box:** `box_header_left`, `box_header_line`, `box_row`, `box_sep`
- **Icons:** `icon_os`, `icon_kernel`, `icon_cpu`, `icon_memory`, … (configurable
  Nerd Font icons)
- **Image logos:** `image_logos` (per-module), `distro_image_logo`

## Custom templates

Place templates in `~/.config/flexfetch/templates/`:

```bash
flexfetch -t my_template
```

Default template path: `~/.config/flexfetch/templates/default.tera`.

## The plain fallback

The minimal build (`--no-default-features`) drops the tera engine to save
~4 MB. It uses a built-in plain renderer that emits fastfetch-style
`├─ Key: value` lines with the same labels the default template uses — readable
output, no template files.

> Note: in minimal builds, a `template` config value is ignored (there is no
> template engine to honor it) and SVG/HTML/PNG exports use the plain renderer.
