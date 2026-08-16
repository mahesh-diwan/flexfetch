# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- 12 new modules (38 → 50 total): `datetime`, `loadavg`, `keyboard`,
  `editor`, `initsystem`, `version`, `bios`, `board`, `chassis`,
  `brightness`, `tpm`, `localip` — all zero-subprocess (read `/proc`, `/sys`,
  DMI, or env directly), matching fastfetch/neofetch parity for the common
  system-info set.

## [0.30.1] - 2026-08-10

### Fixed

- `render_logo` / `render_ascii_logo` were `#[cfg(feature = "tera")]`-gated but called unconditionally from `render()`, breaking the release build (`--no-default-features --features live,image-logos,completions,parallel`). Gates removed; logos now render without the `tera` feature.

## [0.30.0] - 2026-08-10

### Removed

- Plugin system removed end to end: `flexfetch plugin` subcommand, `plugins_dir` config, Lua and WASM plugin crates (`flexfetch-lua`, `flexfetch-wasm`), signed plugin registry, and `registry_sign` publisher example.
- `lua` / `wasm-plugins` feature flags dropped; `base64` moved under the `qr` feature.
- Plugin docs removed from the book, site content, README, and PRODUCT.md; static landing page and module pages updated.

### Changed

- Workspace reduced to `flexfetch-core` + `flexfetch-cli`; default features now `["live", "image-logos", "tera", "parallel", "completions"]`.
- Completions regenerated without plugin subcommands.

## [0.20.0] - 2026-08-07

### Changed

- Over-engineering audit: −2 300 lines, 7 fewer dependencies.
- `theme.rs` 3× duplication → single `THEMES` table.
- `render_info` folded into `render_output`.
- XDG config-dir 5× copy-paste → `tools::config_dir()` helper.
- `export.rs` triplication → shared `render_lines` + `span_to_tag_line`.
- Autotheme hash: FNV-1a → stdlib `DefaultHasher`.
- `base64_decode` → `base64` crate.
- `install.sh`: stripped pacman animation, curl `-#` native progress.
- Completions regenerated, man page updated.

### Removed

- History / daemon / monitor features (`rusqlite`, `notify-rust`).
- `asciinema --record` flag.
- `simd.rs` + `--bench-cpu` / `--bench-memory` flags.
- `flexfetch-tmux` binary.
- `--pixel-logo` flag.
- `CacheConfig` / `[cache] ttl`.
- Notifications and lockfree features (folded into `live`).

## [0.19.0] - 2026-08-07

### Features

- `--flash` mode: ~6 ms uncompressed, ~120 ms UPX'd (measured CachyOS i5-12450H).
- Real download progress (curl progress-bar integration in install.sh).
- PGO pipeline for release builds (profile-guided optimization).

### Bug Fixes

- install.sh hardened: portable version sort (no GNU sort -V), cursor-safety
  EXIT trap, green bold banner with real on-disk size.

## [0.18.0] - 2026-08-02

### Features

- Phase 6 visual overhaul — from system log to system art: tree connectors
  removed; every row is now themed (theme keys/separator/values), keys are
  padded to `display.key_width` via a new Tera `pad` filter, and the default
  theme is now Catppuccin. Added DE/WM/packages/shell/colors rows.
- Adaptive layout: the logo is skipped on narrow terminals (<80 columns, TTY
  gated so exports keep it) and vertically centered; the WM row hides when it
  equals the DE, and Resolution hides when Display already reports it.
- GitHub Action (`--format github`): collapsible `::group::` annotation block
  for CI logs, plus a composite `flexfetch-action` in packaging/.
- Tmux integration: `--tmux-config` prints a ~/.tmux.conf snippet and a new
  pure-std `flexfetch-tmux` helper binary.
- Hardware database: `hardware_db.rs` maps PCI/USB vendor/device IDs to friendly
  names (bundled seed + `--update-db` refresh); the GPU module resolves
  `/sys/class/drm` IDs through it.
- Distribution: Nix flake + Home Manager module, AUR PKGBUILD, Homebrew
  formula, scratch container image, MIT LICENSE.

### Bug Fixes

- Disk usage no longer renders a doubled percent sign (`82%%` → `82%`), and the
  duplicate-mount dedup check was aligned to the corrected entry format.

## [0.17.0] - 2026-08-02

### Features

- Phase 4.1 zero-spawn collectors — cold start 5.5 s → 686 ms (242 ms with the
  publicip cache warm): kernel, packages, wm, disk, health, network, wifi,
  publicip, and cpuusage no longer spawn subprocesses (raw `/proc`/`/sys`
  parsing, `libc::statvfs`/`getifaddrs`, `nmcli --rescan no`, raw `TcpStream`
  HTTP for public IP). `--benchmark` now reports cold start + per-module timing.
- `--update` — self-update via the idempotent install script (skips when current).
- `--doctor` — environment diagnostics (TTY/truecolor/Nerd Font/config/collectors).
- `--hook bash|zsh|fish` — cd-into-git-repo prompt snippets.
- `--qr` / `--import-qr` — QR config sharing (base64+zstd, `qr` feature, opt-in).
- New modules: `container`, `fsdeep`, `wallpaper`, `weather` (+ terminal
  fingerprinting) and SIMD helpers behind feature gates.
- Installer hardening: SHA-256 verification (fail-closed), `.bak` backup before
  overwrite, temp cleanup on success and failure paths.
- Release pipeline now emits per-artifact `.sha256` checksums and uploads
  `install.sh` alongside the tarballs; one-line release install works.

### Performance

- Cold start gate: `perf-gate` CI job measures `flexfetch --minimal` with
  hyperfine and fails if the mean exceeds 10 ms.
- `qr-feature` CI job exercises the opt-in QR build (tests + clippy).
- Binary diet (earlier batch): tera + rayon gated behind `tera`/`parallel`
  features — the release binary (~2 MB, `live,image-logos,completions`) and
  the minimal build (~1.5 MB, `--no-default-features`) are far smaller.

### Bug Fixes

- macOS cross-build fixes: `network.rs` type inference + dead-code gating in
  `cpu.rs`/`cpuusage.rs`/`cpucache.rs`.
- Release workflow: download zig directly (setup-zig fetched a wrong filename),
  skip artifact upload on tagless validation runs, install zig to `~/.local`.

### Documentation

- mdBook docs site (modules, templates, plugins, themes, CLI reference, FAQ).
- mtime-based hot-reload for `--watch`/`--live` (no external watcher).
- `flexfetch completions bash|zsh|fish` generator (`completions` feature).

### Documentation

- README: accurate counts (27 theme presets, 527+ ASCII logos), release-based
  one-line install, Update/Doctor/Hook section, corrected comparison table.
- Man page updated for v0.17.0 with the new flags.
- ROADMAP: 4.1 and 4.11 marked done, 4.17 partially done.
- `cliff.toml` added for git-cliff changelog generation.

<!-- generated by git-cliff -->
