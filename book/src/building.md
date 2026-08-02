# Building

## Feature flags

| Feature       | Default | Adds                                                        |
| ------------- | ------- | ----------------------------------------------------------- |
| `lua`         | ✅      | Lua plugin support (mlua, vendored 5.4 — needs a C compiler) |
| `live`        | ✅      | `--live` dashboard + `--wizard` (ratatui/crossterm)          |
| `image-logos` | ✅      | Sixel/block image logos + `--export png` (image crate)       |
| `tera`        | ✅      | Tera template engine (plain fallback renderer without it)    |
| `parallel`    | ✅      | Rayon parallel module collection (sequential without it)     |
| `completions` | ✅      | `completions <shell>` subcommand (clap_complete)             |
| `music`       | —       | MPRIS via pure-Rust zbus (else `dbus-send` shell-out)        |

## Common builds

```bash
cargo build --release                          # everything (default)
cargo build --release --no-default-features    # minimal ~1.5 MB: no lua/live/tera/rayon
cargo build --release --no-default-features --features live,image-logos
                                               # what the release pipeline ships
cargo test                                     # workspace tests
```

## Binary size

The feature flags exist so the binary can be trimmed. Measured on Linux x86_64
(release, LTO):

| Build                | Size  |
| -------------------- | ----- |
| default (all)        | ~6.3 MB |
| release pipeline     | ~2.0 MB |
| minimal (none)       | ~1.5 MB |

The minimal build drops tera (~4 MB of the weight) for the plain renderer and
rayon for sequential loops. This is what the `< 3 MB` diet target refers to.
