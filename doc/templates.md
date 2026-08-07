# FlexFetch Template Guide

## Overview

FlexFetch uses [Tera](https://tera.netlify.app/) templates (Jinja2/Django syntax) for customizable output.

## Quick Start

1. Generate default config: `flexfetch --gen-config`
2. Edit template: `~/.config/flexfetch/templates/default.tera`
3. Test: `flexfetch -t my_template`

## Template Syntax

### Variables

```tera
{{ variable }}                    # Simple variable
{{ os.pretty_name }}              # Nested access
{{ memory.used | default("N/A") }} # With default filter
```

### Conditionals

```tera
{% if cpu %}
CPU: {{ cpu.model }}
{% endif %}

{% if battery.percent %}
Battery: {{ battery.percent }}%
{% else %}
No battery
{% endif %}
```

### Loops

```tera
{% for mount in disk %}
Disk: {{ mount }}
{% endfor %}

{% for iface in network %}
{{ iface.name }}: {{ iface.ipv4 }}
{% endfor %}
```

### Filters

```tera
{{ value | upper }}               # Uppercase
{{ value | lower }}               # Lowercase
{{ value | truncate(length=20) }} # Truncate
{{ value | default("N/A") }}     # Default value
```

## Available Variables

### System Info

| Variable          | Type   | Description                  |
| ----------------- | ------ | ---------------------------- |
| `os.pretty_name`  | string | OS name (e.g., "Arch Linux") |
| `os.name`         | string | OS ID                        |
| `kernel`          | string | Kernel version               |
| `host`            | string | Hostname                     |
| `uptime`          | string | Uptime                       |
| `locale.lang`     | string | Locale                       |
| `locale.encoding` | string | Encoding                     |

### Hardware

| Variable              | Type   | Description   |
| --------------------- | ------ | ------------- |
| `cpu.model`           | string | CPU model     |
| `cpu.cores`           | string | Core count    |
| `cpu.freq_mhz`        | string | Frequency     |
| `cpu.temp`            | string | Temperature   |
| `memory.used`         | string | Used memory   |
| `memory.total`        | string | Total memory  |
| `memory.percent`      | string | Usage percent |
| `memory.swap_used`    | string | Swap used     |
| `memory.swap_total`   | string | Swap total    |
| `memory.swap_percent` | string | Swap percent  |

### Storage

| Variable | Type  | Description           |
| -------- | ----- | --------------------- |
| `disk`   | array | List of mount strings |
| `gpu`    | array | List of GPU strings   |

### Network

| Variable         | Type   | Description        |
| ---------------- | ------ | ------------------ |
| `network`        | array  | Network interfaces |
| `network[].name` | string | Interface name     |
| `network[].ipv4` | string | IPv4 address       |
| `network[].ipv6` | string | IPv6 address       |
| `network[].mac`  | string | MAC address        |

### Display

| Variable       | Type   | Description        |
| -------------- | ------ | ------------------ |
| `theme_keys`   | string | Key color escape   |
| `theme_values` | string | Value color escape |
| `theme_sep`    | string | Separator color    |
| `theme_reset`  | string | Color reset        |
| `theme_title`  | string | Title color        |

### Icons (Nerd Font)

| Variable       | Default | Description  |
| -------------- | ------- | ------------ |
| `icon_os`      | 󰟀       | OS icon      |
| `icon_kernel`  | 󰌽       | Kernel icon  |
| `icon_host`    | 󰟀       | Host icon    |
| `icon_uptime`  | 󰅐       | Uptime icon  |
| `icon_cpu`     | 󰍛       | CPU icon     |
| `icon_gpu`     | 󰢮       | GPU icon     |
| `icon_memory`  | 󰉀       | Memory icon  |
| `icon_disk`    | 󰋊       | Disk icon    |
| `icon_network` | 󰩟       | Network icon |
| `icon_battery` | 󰁹       | Battery icon |

## Example Templates

### Minimal

```tera
{{ os.pretty_name }} | {{ kernel }} | {{ uptime }}
```

### With Icons

```tera
├─{{ icon_os }} OS: {{ os.pretty_name }}
├─{{ icon_kernel }} Kernel: {{ kernel }}
├─{{ icon_cpu }} CPU: {{ cpu.model }}
╰─{{ icon_memory }} Memory: {{ memory.used }} / {{ memory.total }}
```

### Conditional Sections

```tera
{% if os %}
OS: {{ os.pretty_name }}
{% endif %}
{% if cpu %}
CPU: {{ cpu.model }} ({{ cpu.cores }} cores)
{% endif %}
{% if memory %}
Memory: {{ memory.used }} / {{ memory.total }} ({{ memory.percent }})
{% endif %}
```

## Custom Filters

FlexFetch provides these custom filters:

### palette_display

```tera
{{ colors | palette_display(style="blocks") }}
{{ colors | palette_display(style="squares") }}
{{ colors | palette_display(style="dots") }}
```

### progress_bar

```tera
{{ memory.percent | progress_bar(width=10) }}
```

## Tips

1. Use `{%- if %}` (with dash) to trim whitespace
2. Use `| default("N/A")` for optional values
3. Test with `flexfetch -t template.tera --debug`
4. Check syntax at https://tera.netlify.app playground
