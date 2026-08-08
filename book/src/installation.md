# Installation

## One-line install

```bash
curl -fsSL https://raw.githubusercontent.com/mahesh-diwan/flexfetch/main/install.sh | sh
```

Installs the latest binary (~2 MB, statically linked on Linux) from
[GitHub Releases](https://github.com/mahesh-diwan/flexfetch/releases). Requires
`curl` + `sudo`. Works on Linux and macOS.

The install script:

- Detects your architecture (x86_64, aarch64)
- Downloads the prebuilt binary from the latest release
- Installs to `/usr/local/bin/flexfetch`
- Generates shell completions for bash, zsh, and fish

Prebuilt binaries include the live dashboard, image logos, and shell-completion
generation, but **exclude Lua** (that keeps the pure-Rust binary lean).

## From source

Includes Lua plugin support:

```bash
cargo install --git https://github.com/mahesh-diwan/flexfetch
```

Default builds compile vendored Lua 5.4, so a C compiler is required.

## Verify

```bash
flexfetch --version
flexfetch --theme dracula
flexfetch -f json | head -5
```

## Shell completions

After installing, generate completions:

```bash
flexfetch completions bash > ~/.bash_completion.d/flexfetch
flexfetch completions zsh  > ~/.zsh/completions/_flexfetch
flexfetch completions fish > ~/.config/fish/completions/flexfetch.fish
```

## Self-update

If you installed via the install script:

```bash
flexfetch --update
```

Checks the latest GitHub release and re-runs the install script if a newer
version exists.
