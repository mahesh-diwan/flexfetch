# Installation

## One-line install (official)

The only supported way to install flexfetch is the official installer:

```bash
curl -fsSL https://github.com/mahesh-diwan/flexfetch/releases/latest/download/install.sh | sh
```

It installs the latest release binary (the full default feature build,
statically linked on Linux) from
[GitHub Releases](https://github.com/mahesh-diwan/flexfetch/releases).
Requires `curl` + `sudo`. Works on Linux and macOS.

The installer:

- Detects your architecture (x86_64, aarch64, armv7 on Linux; arm64 and x86_64 on macOS)
- Downloads the prebuilt binary from the latest release
- Verifies the SHA-256 checksum of the archive and **aborts on mismatch**
  (verification needs a `sha256sum`/`shasum` tool; it is skipped with a
  warning if no checksum tool is available)
- Installs to `/usr/local/bin/flexfetch`

Prebuilt binaries include the live dashboard, image logos, and the
`flexfetch completions` subcommand.

> **Note:** `cargo install` is not an install path — the repo is a Rust
> workspace, and the officially supported way to get flexfetch is the
> installer above.

## Verify

```bash
flexfetch --version
flexfetch --theme dracula
flexfetch -f json | head -5
```

## Shell completions

flexfetch ships completion definitions for bash, zsh, and fish. After
installing, generate them with the built-in subcommand:

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
