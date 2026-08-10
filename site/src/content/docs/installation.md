---
title: Installation
description: How to install flexfetch on Linux and macOS
order: 2
---

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
generation.

## From source

```bash
cargo install --git https://github.com/mahesh-diwan/flexfetch
```

## Uninstall

Remove the binary and generated files:

```bash
sudo rm /usr/local/bin/flexfetch
rm -rf ~/.config/flexfetch
```

Shell completions are removed automatically by the install script on
uninstall. If you copied them manually:

```bash
sudo rm /etc/bash_completion.d/flexfetch   # bash (Ubuntu/Debian)
sudo rm /usr/share/zsh/vendor-completions/_flexfetch  # zsh
rm ~/.config/fish/completions/flexfetch.fish  # fish
```

## Verify

```bash
flexfetch --version
flexfetch --theme dracula
flexfetch -f json | head -5
```

You should see the version number, a themed logo output, and valid JSON.
If `flexfetch` is not found, check that `/usr/local/bin` is in your `$PATH`.

## Updating

```bash
flexfetch --update
```

This checks the latest GitHub release and re-runs the install script if a
newer version exists. For source builds:

```bash
cargo install --git https://github.com/mahesh-diwan/flexfetch
```

## Troubleshooting

**Binary not found after install** — add `/usr/local/bin` to your PATH:

```bash
export PATH="/usr/local/bin:$PATH"
```

Add the line to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) to make
it permanent.

**Permission denied** — the install script needs `sudo` to write to
`/usr/local/bin`. Run it with `sudo` or use `cargo install` to install to
`~/.cargo/bin` instead.

**"command not found: flexfetch" in a new terminal** — restart your shell or
source your profile: `source ~/.bashrc`.

**Wrong architecture** — the install script auto-detects x86_64 and aarch64.
If you're on a different arch, build from source with `cargo install`.

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
