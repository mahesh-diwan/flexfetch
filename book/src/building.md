# Building

## Feature flags

| Feature        | Default | Adds                                                         |
| -------------- | ------- | ------------------------------------------------------------ |
| `lua`          | ✅      | Lua plugin support (mlua, vendored 5.4 — needs a C compiler) |
| `live`         | ✅      | `--live` dashboard + `--wizard` (ratatui/crossterm)          |
| `image-logos`  | ✅      | Sixel/block image logos + `--export png` (image crate)       |
| `tera`         | ✅      | Tera template engine (plain fallback renderer without it)    |
| `parallel`     | ✅      | Rayon parallel module collection (sequential without it)     |
| `completions`  | ✅      | `completions <shell>` subcommand (clap_complete)             |
| `music`        | —       | MPRIS via pure-Rust zbus (else `dbus-send` shell-out)        |
| `qr`           | —       | QR code config export/import (rqrr)                          |
| `auto-theme`   | —       | Derive theme from wallpaper colors                           |
| `wasm-plugins` | —       | WASM plugin runtime (wasmtime, sandboxed)                    |

## Common builds

```bash
cargo build --release                          # everything (default)
cargo build --release --no-default-features    # minimal ~1.7 MB: no lua/live/tera/rayon
cargo build --release --no-default-features --features live,image-logos
                                               # what the release pipeline ships
cargo test                                     # workspace tests
```

## Binary size

The feature flags exist so the binary can be trimmed. Measured on Linux x86_64
(release, LTO):

| Build            | Size    |
| ---------------- | ------- |
| default (all)    | ~6.9 MB |
| release pipeline | ~2.0 MB |
| minimal (none)   | ~1.7 MB |

The minimal build drops tera (~4 MB of the weight) for the plain renderer and
rayon for sequential loops.

## Performance

Measured on CachyOS, i5-12450H:

| Metric                    | Value   |
| ------------------------- | ------- |
| Flash mode (uncompressed) | ~5-7 ms |
| Flash mode (UPX'd)        | ~120 ms |
| Module collection         | <10 ms  |
| Template render           | <5 ms   |

## Tests

```bash
cargo test                                     # all tests
cargo test --no-default-features               # minimal build tests
cargo clippy -- -W clippy::all                 # lint
```

77 tests across 13 test suites.
