# Building

## Feature flags

| Feature       | Default | Adds                                                      |
| ------------- | ------- | --------------------------------------------------------- |
| `live`        | ✅      | `--live` dashboard + `--wizard` (ratatui/crossterm)       |
| `image-logos` | ✅      | Sixel/block image logos + `--export png` (image crate)    |
| `tera`        | ✅      | Tera template engine (plain fallback renderer without it) |
| `parallel`    | ✅      | Rayon parallel module collection (sequential without it)  |
| `completions` | ✅      | `completions <shell>` subcommand (clap_complete)          |
| `music`       | —       | MPRIS via pure-Rust zbus (else `dbus-send` shell-out)     |
| `qr`          | —       | QR code config export/import (rqrr)                       |
| `auto-theme`  | —       | Derive theme from wallpaper colors                        |

## Common builds

```bash
cargo build --release                          # everything (default)
cargo build --release --no-default-features    # minimal (~1.75 MB)
cargo build --release --no-default-features --features live,image-logos,completions,parallel
                                               # what the release pipeline ships
cargo test                                     # workspace tests
```

## Binary size

The feature flags exist so the binary can be trimmed. Measured on Linux x86_64
(release, LTO):

| Build            | Size     |
| ---------------- | -------- |
| default (all)    | ~6.8 MB  |
| release pipeline | ~2.4 MB  |
| minimal (none)   | ~1.75 MB |

The minimal build drops tera (~4 MB of the weight) for the plain renderer,
rayon for sequential loops, and the live/completions subcommands.

## Performance

Measured with `flexfetch --benchmark` on CachyOS, i5-12450H (release build;
values vary run to run):

| Metric            | Value            |
| ----------------- | ---------------- |
| Module collection | ~26 ms (23–28 ms) |
| Template render   | ~0.33 ms (0.27–0.41 ms) |
| Full pipeline     | ~130 ms (cold start dominates; ~57 ms warm) |

Individual modules run in microseconds (e.g. `cpu` ~0.65 ms, `os` ~30 µs);
`--benchmark` prints the full per-module breakdown.

## Tests

```bash
cargo test                                     # all tests
cargo test --no-default-features               # minimal build tests
cargo clippy -- -W clippy::all                 # lint
```

153 tests across 7 test suites.
