# Installation

## One-line install

```bash
curl -fsSL https://raw.githubusercontent.com/mahesh-diwan/flexfetch/main/install.sh | sh
```

Installs the latest binary (~2 MB, statically linked on Linux) from
[GitHub Releases](https://github.com/mahesh-diwan/flexfetch/releases). Requires
`curl` + `sudo`. Works on Linux and macOS.

Prebuilt binaries include the live dashboard, image logos, and shell-completion
generation, but **exclude Lua** (that keeps the pure-Rust binary lean).

## From source

Includes Lua plugin support:

```bash
cargo install --git https://github.com/mahesh-diwan/flexfetch
```

Default builds compile vendored Lua 5.4, so a C compiler is required.

## Try it

```bash
flexfetch --theme dracula
flexfetch -f json
flexfetch -m "os:kernel:uptime"
flexfetch --list-modules
flexfetch --gen-config
```
