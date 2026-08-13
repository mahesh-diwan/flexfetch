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
values vary run to run). Slow modules (wifi, display, packages, bluetooth,
media, publicip) reuse a 60 s cache, so repeated runs are fastest:

| Metric            | Value            |
| ----------------- | ---------------- |
| Full run, warm cache | ~9 ms (6–11 ms) |
| Full run, cold cache | ~14 ms (13–16 ms) |
| Module collection (run_selected) | ~3 ms (2.9–4.0 ms) |
| Template render   | ~0.5 ms (0.47–0.76 ms) |

Individual modules run in microseconds to ~1 ms (e.g. `cpu` ~1.1 ms,
`os` ~0.05 ms, `wifi` 0.02 ms warm / ~3 ms cold); `--benchmark` prints the
full per-module breakdown. The `cache_ttl` config key (default 60 s) controls
how long slow-module results are reused between runs.

## Tests

```bash
cargo test                                     # all tests
cargo test --no-default-features               # minimal build tests
cargo clippy -- -W clippy::all                 # lint
```

153 tests across 7 test suites.
