<p align="center">
  <img src="assets/default.svg" width="720" alt="flexfetch terminal output">
</p>

<h1 align="center">flexfetch</h1>

<p align="center">
  <em>Your system info, your rules.</em><br>
  Lua plugins · Tera templates · 5 theme presets · Written in Rust
</p>

<p align="center">
  <a href="#installation"><kbd>Install in one line →</kbd></a>
</p>

<p align="center">
  <a href="https://github.com/mahesh-diwan/flexfetch/releases/latest"><img src="https://img.shields.io/github/v/release/mahesh-diwan/flexfetch?style=flat&label=release" alt="release"></a>
  <a href="https://github.com/mahesh-diwan/flexfetch/actions/workflows/release.yml"><img src="https://img.shields.io/github/actions/workflow/status/mahesh-diwan/flexfetch/release.yml?style=flat&label=build" alt="build"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/mahesh-diwan/flexfetch?style=flat&color=blue" alt="license"></a>
  <img src="https://img.shields.io/github/repo-size/mahesh-diwan/flexfetch?style=flat&label=size" alt="size">
</p>

<br>

## Installation

```bash
curl -fsSL https://raw.githubusercontent.com/mahesh-diwan/flexfetch/main/install.sh | sh
```

Installs latest binary (~5 MB, statically linked) from [GitHub Releases](https://github.com/mahesh-diwan/flexfetch/releases). Requires `curl` + `sudo`. Works on Linux and macOS.

**From source** (includes Lua plugin support):

```bash
cargo install --git https://github.com/mahesh-diwan/flexfetch
```

**Try it:**

```bash
flexfetch --theme dracula
flexfetch -f json
flexfetch -m "os:kernel:uptime"
flexfetch --list-modules
flexfetch --gen-config
```

<p align="center">
  <img src="assets/json.svg" width="720" alt="flexfetch JSON output">
</p>

<br>

---

## Why flexfetch?

Every system info tool shows the same thing — OS, kernel, uptime, done. flexfetch gives you three things no other tool does:

|     | Feature             | What it means                                                                                                                      |
| --- | ------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| 🔌  | **Lua plugins**     | Write info modules in Lua. Drop a `.lua` file in `~/.config/flexfetch/plugins/` and it appears in output. No compilation. No Bash. |
| 📝  | **Tera templates**  | Jinja2-style templates. Variables, loops, conditionals. Default template renders side-by-side logo + info with right-aligned keys. |
| 🎭  | **5 theme presets** | Catppuccin, Dracula, Nord, Gruvbox, Tokyo Night. Switch with `--theme`. Per-field overrides with named colors.                     |
| ⚡  | **Rust + Rayon**    | Parallel detection. Static binary, zero runtime deps. ~5 MB. No Python, no Node, no Bash.                                          |

<br>

## Comparison

|                    | flexfetch              | neofetch       | fastfetch     | pfetch   |
| ------------------ | ---------------------- | -------------- | ------------- | -------- |
| **Language**       | Rust                   | Bash           | C             | sh       |
| **Lua plugins**    | ✅                     | —              | —             | —        |
| **Tera templates** | ✅                     | —              | —             | —        |
| **Theme presets**  | ✅ 5 + named overrides | built-in       | JSON5 presets | 3 env    |
| **Parallel**       | ✅ Rayon               | —              | ✅            | —        |
| **Output formats** | text, JSON             | text           | text, JSON    | text     |
| **Config**         | TOML                   | —              | JSON5         | env vars |
| **ASCII logos**    | 7 + generic            | ~150           | ~200          | small    |
| **Binary size**    | 5 MB                   | ~1 KB (script) | 2 MB          | 5 KB     |
| **Runtime deps**   | none                   | Bash + utils   | none          | sh       |

<br>

---

## Themes

5 presets. Same output, dramatically different look. Switch at runtime:

```bash
flexfetch --theme nord
flexfetch --theme tokyo-night
flexfetch --theme gruvbox
```

<p align="center">
  <img src="assets/themes.svg" width="720" alt="flexfetch theme comparison — 5 themes, same output">
</p>

Override any preset with named colors:

```toml
[display]
theme = "catppuccin"
color_keys = "yellow"
color_values = "green"
color_sep = "red"
```

Colors resolve from a named set (`black`/`red`/`green`/`yellow`/`blue`/`magenta`/`cyan`/`white` + `bright_*` + `bold`). Or use raw ANSI escapes: `"\u001b[92m"`.

| Theme         | Keys   | Values |
| ------------- | ------ | ------ |
| `catppuccin`  | pink   | cyan   |
| `dracula`     | pink   | cyan   |
| `nord`        | blue   | green  |
| `gruvbox`     | yellow | green  |
| `tokyo-night` | blue   | cyan   |

<br>

---

## Lua Plugins

Write custom info modules in Lua 5.4. No compilation, no Bash scripting.

```lua
-- ~/.config/flexfetch/plugins/user.lua
return {
    name = "user",
    collect = function(ctx)
        local user = ctx.get_env("USER")
        local shell = ctx.get_env("SHELL")
        return { value = user .. " (" .. shell .. ")" }
    end
}
```

**Plugin API:**

| Function               | Returns | Description              |
| ---------------------- | ------- | ------------------------ |
| `ctx.read_file(path)`  | string  | Read file contents       |
| `ctx.run_command(cmd)` | string  | Execute shell command    |
| `ctx.get_env(key)`     | string  | Get environment variable |

List plugins: `flexfetch --list-plugins`. Disable Lua at build time: `cargo build --release --no-default-features`.

Built with [mlua](https://github.com/khvzak/mlua) 0.10 (Lua 5.4).

<br>

---

## Templates

Full control over output layout with [Tera](https://tera.netlify.app/) (Jinja2/Django syntax). Default template renders side-by-side logo + info with right-aligned labels.

**Context variables:**

- **Scalars:** `kernel`, `host`, `uptime`
- **Maps:** `os.pretty_name`, `locale.lang`, `shell.name`, `cpu.model`, `memory.used`
- **Theme:** `theme_keys`, `theme_values`, `theme_reset`, `theme_title`, `theme_sep`
- **Display:** `display_separator`, `display_key_width`

Place custom templates in `~/.config/flexfetch/templates/`:

```bash
flexfetch -t my_template
```

Default template path: `~/.config/flexfetch/templates/default.tera`.

<br>

---

## Custom Modules (no code)

Define info sources inline in config. No plugin needed.

```toml
[custom]
my_temp = { command = "sensors | grep temp1", label = "Temp" }
sys_load = { command = "uptime | awk -F'load average:' '{print $2}'", label = "Load" }
```

Each custom module runs the shell command on every fetch and displays the result.

<br>

---

## Configuration

Config at `~/.config/flexfetch/config.toml`. Generate with `flexfetch --gen-config`.

```toml
modules = ["title", "separator", "os", "host", "kernel", "uptime",
           "shell", "cpu", "memory", "colors"]

[display]
separator = ": "
key_width = 8
theme = "catppuccin"

[cache]
ttl = 60               # seconds, 0 to disable
```

Cache is a JSON file at `~/.cache/flexfetch/`. Reduces repeated disk reads. TTL = 60s by default.

<br>

---

## Output Formats

| Format | Use case           |
| ------ | ------------------ |
| `text` | Terminal (default) |
| `json` | Scripts, tooling   |

JSON mode disables ASCII art and themes. Output is structured for parsing:

```bash
flexfetch -f json | jq '.os.name'
```

<br>

---

## Modules

| Module                                                                                                                   | Status |
| ------------------------------------------------------------------------------------------------------------------------ | ------ |
| `os`, `host`, `kernel`, `uptime`, `locale`                                                                               | ✅     |
| `colors`                                                                                                                 | ✅     |
| `cpu`, `memory`, `disk`, `gpu`, `network`, `battery`, `processes`, `packages`, `shell`, `terminal`, `de`, `wm`, `custom` | 🚧     |
| `title`, `separator`                                                                                                     | 📐     |

All modules run in parallel via Rayon. Stubs return empty — [PRs welcome](https://github.com/mahesh-diwan/flexfetch/pulls).

<br>

---

## Logo Support

flexfetch detects distro from `/etc/os-release` and renders ASCII art next to info.

| Distro  | Lines | Matches                                               |
| ------- | ----- | ----------------------------------------------------- |
| Arch    | 5     | arch, cachyos, endeavouros, arcolinux, artix, manjaro |
| Debian  | 5     | debian, raspbian                                      |
| Ubuntu  | 5     | ubuntu, linuxmint, pop, elementary, zorin             |
| Fedora  | 6     | fedora                                                |
| NixOS   | 5     | nixos                                                 |
| macOS   | 6     | auto-detected                                         |
| Generic | 6     | anything else                                         |

<br>

---

## Building

```bash
cargo build --release                     # all features
cargo build --release --no-default-features  # without Lua
cargo test
```

<br>

---

## FAQ

**How is this different from neofetch/fastfetch?** Lua plugins, Tera templates, and theme presets — no other tool has all three.

**How do I add info that isn't built in?** Two ways: `[custom]` config section (shell commands) or a Lua plugin.

**Why are some modules empty?** 13 are stubs — they compile but need implementation. See the [Module trait](flexfetch-core/src/module.rs).

**Does it work on macOS?** Yes. OS detection via `sw_vers`. macOS logo auto-detected.

<br>

---

<p align="center">
  <b>flexfetch</b> — MIT licensed<br>
  <sub>Inspired by <a href="https://github.com/dylanaraps/neofetch">neofetch</a>, <a href="https://github.com/fastfetch-cli/fastfetch">fastfetch</a>, and <a href="https://github.com/dylanaraps/pfetch">pfetch</a></sub><br>
  <sub>Built with <a href="https://www.rust-lang.org/">Rust</a>, <a href="https://tera.netlify.app/">Tera</a>, <a href="https://github.com/khvzak/mlua">mlua</a>, <a href="https://github.com/rayon-rs/rayon">Rayon</a></sub><br>
  <br>
  <a href="https://github.com/mahesh-diwan/flexfetch/stargazers">⭐ Star on GitHub</a>
</p>
