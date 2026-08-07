<p align="center">
  <img src="assets/default.svg" width="720" alt="flexfetch terminal output">
</p>

<h1 align="center">flexfetch</h1>

<p align="center">
  <em>A fast, beautiful system information tool.</em><br>
  Lua + WASM plugins · Tera templates · 27 themes · 527+ logos · Rust
</p>

<p align="center">
  <a href="#installation"><kbd>Install in one line →</kbd></a>
</p>

<p align="center">
  <a href="https://github.com/mahesh-diwan/flexfetch/releases/latest"><img src="https://img.shields.io/github/v/release/mahesh-diwan/flexfetch?style=flat&label=release" alt="release"></a>
  <a href="https://github.com/mahesh-diwan/flexfetch/actions/workflows/release.yml"><img src="https://img.shields.io/github/actions/workflow/status/mahesh-diwan/flexfetch/release.yml?style=flat&label=build" alt="build"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/mahesh-diwan/flexfetch?style=flat&color=blue" alt="license"></a>
</p>

---

## Installation

```bash
curl -fsSL https://github.com/mahesh-diwan/flexfetch/releases/latest/download/install.sh | sh
```

From source (includes Lua plugins):

```bash
cargo install --git https://github.com/mahesh-diwan/flexfetch
```

---

## Quick Start

```bash
flexfetch                     # default output
flexfetch --flash             # fastest possible fetch
flexfetch --minimal           # minimal module set
flexfetch --theme catppuccin  # switch theme
```

---

## Features

- **Lua plugins** — write info modules in Lua 5.4, drop in `~/.config/flexfetch/plugins/`
- **WASM plugins** — sandboxed, any language that compiles to `wasm32-unknown-unknown`
- **Tera templates** — Jinja2-style templates for full layout control
- **27 theme presets** — Catppuccin, Dracula, Nord, Gruvbox, Tokyo Night, and more
- **527+ ASCII logos** — auto-detected per distro
- **Live dashboard** — real-time CPU, memory, network, top processes
- **Diff mode** — compare system state between two points in time
- **QR sharing** — generate a QR code of your fetch for easy sharing
- **SSH remote fetch** — fetch info from remote hosts in parallel
- **Flash mode** — fastest one-shot fetch with no config or plugins
- **Custom modules** — define info sources inline in config (shell commands)
- **Export formats** — text, JSON, Markdown, SVG, HTML, PNG, GitHub Actions
- **Shell completions** — bash, zsh, fish
- **Health score** — 0–100 system health from disk, swap, load, battery
- **Smart fetch** — context-aware info (git branch, project type, container, SSH)
- **Watch mode** — periodic refresh for dashboards
- **Config wizard** — interactive TUI setup with live preview
- **Plugin registry** — search, install, and update plugins from a hosted registry

---

## Configuration

Edit `~/.config/flexfetch/config.toml` or run `flexfetch --wizard`.

```toml
modules = ["title", "separator", "os", "host", "kernel", "uptime", "cpu", "memory"]

[display]
theme = "catppuccin"
separator = ": "
```

📖 Full docs: [mdBook site](https://mahesh-diwan.github.io/flexfetch/)

---

## Building

See [docs/building.md](book/src/building.md).

```bash
cargo build --release          # full build
cargo test
```

---

## Links

- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)

---

## License

[MIT](LICENSE) — inspired by [neofetch](https://github.com/dylanaraps/neofetch), [fastfetch](https://github.com/fastfetch-cli/fastfetch), and [pfetch](https://github.com/dylanaraps/pfetch).
