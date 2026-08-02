# flexfetch v2.0 — Roadmap

> Living document. Update statuses as work lands. When continuing work, consult this file
> to pick the next task, and check the **Rejected / decisions** section before proposing
> anything that was already evaluated.

Status legend: ✅ done · 🟡 in progress · ⬜ pending · 🚫 rejected · ⚠️ partial

---

## Phase 0 — Foundation

### 0.1 Workspace structure — ✅ (already in place, no change needed)
Existing workspace (`flexfetch-core` / `flexfetch-cli` / `flexfetch-lua`) already provides
independent compilation and clean layering. The plan's `crates/ff-*` rename was evaluated
and rejected (see Rejected section) — it would break every import, CI, install script, and
test for zero functional gain.

### 0.2 Binary size diet — ✅ done (Aug 2026)
Goal: shrink the binary from 8.8 MB → < 3 MB (full) / < 800 KB (minimal).

Result so far: **8.8 MiB → 6.0 MiB (−32%)** (6.29 MB decimal) on native glibc; ARM musl builds are even smaller (see 0.3 for per-target decimal sizes).

What was done:
- `image` crate gated to `default-features = false, features = ["png"]` — was pulling
  every decoder (exr, avif, gif, webp, tiff, qoi…). All logos and the PNG export are PNG.
- Removed unused `chrono` dependency (zero references in the repo).
- `clap` trimmed to minimal features (`std, color, help, usage, error-context, derive`).
- Added proper `lua` feature flag to `flexfetch-cli` (`default = ["lua"]`) so
  `cargo build --release --no-default-features` now works as documented.
- Removed dead constants (`FF_COLORS`, `RESET`) from generated `fastfetch_logos.rs` (the generator script doesn't emit them, so regeneration won't resurrect the warnings).
- README size claims corrected (5 MB → 6 MB; it was actually 8.8 MB).

Verify: `stat -c%s target/release/flexfetch` → ~6.29 MB. All 25 tests pass.

**Remaining diet work (DONE Aug 2026):** the image-dependent code (sixel/block image
logos + PNG export) is now gated behind a new `image-logos` feature (default on), so
`--no-default-features` drops the `image` crate entirely: minimal 6.30 MB → 6.04 MB,
full unchanged at 6.50 MB (release ships `--features live,image-logos`). When the
feature is off, sixel/block logos fall back to ASCII (kitty/iTerm2 still work — they
only base64 raw bytes) and `--export png` returns a clear "requires the image-logos
feature" error. Also consider a musl static build (already produced by 0.3 CI at
~4.4–6.4 MB).

**Diet complete (Aug 2026):** tera + rayon are now gated behind new `tera`/`parallel`
features (default on). `--no-default-features` drops tera (a plain `├─ Key: value`
fallback renderer in template.rs replaces it — no template filtering, exports use the
plain renderer) and rayon (sequential collection). Result: **minimal 1.53 MB** (under
the 3 MB target), default full build 6.33 MB, release pipeline
(`--no-default-features --features live,image-logos,completions`) 2.02 MB. Release size
gate tightened 8 MB → 4 MB. The plain renderer is covered by the template integration
tests in the minimal-build CI job (`cargo test -p flexfetch-core --no-default-features`).

### 0.3 Cross-compile build pipeline — ✅ done (Aug 2026)
Fixed a real bug: `install.sh` downloaded `flexfetch-linux-{amd64,aarch64,armv7}.tar.gz`
that the old CI never produced (only x86_64 Linux was built), and macOS install was
completely broken (URL hardcoded to `linux`).

What was done:
- `.github/workflows/release.yml` rewritten as a 5-target matrix:
  - Linux: `x86_64`, `aarch64`, `armv7` (musl, fully static) via `cargo-zigbuild`
    (setup-zig + `taiki-e/install-action@cargo-zigbuild`).
  - macOS: `aarch64` + `x86_64` both on one `macos-latest` (arm64) runner — aarch64 native,
    x86_64 cross-compiled via system clang (pure-Rust tree needs no C toolchain).
  - All targets built with `--no-default-features` (mlua is dead code in the binary — LTO
    strips it — so excluding it changes nothing functionally but keeps the tree pure Rust).
  - Size gate (Linux, uncompressed binary, 8 MB regression guard — not a diet target),
    UPX compression (Linux only, non-fatal), `workflow_dispatch` manual trigger.
- `mlua` switched to `vendored` in `flexfetch-core` + `flexfetch-lua` — no system Lua
  headers needed; note default builds now compile vendored Lua C code (need a C compiler).
- `install.sh`: OS detection (`linux`/`macos`), arch mapping to new artifact names.
- `flexfetch-cli --version` features list is now cfg-gated (no longer falsely claims `lua`
  on minimal builds).
- README notes: prebuilt binaries exclude Lua; source builds include it.Locally validated: native 6.29 MB · x86_64-musl 6.40 MB (static-pie) · aarch64-musl 4.69 MB (static) · armv7-musl 4.38 MB (static) — all link.
**GitHub matrix validation — ✅ DONE (Aug 2026):** `workflow_dispatch` runs proved the
macOS aarch64 + x86_64 cross-builds (both green across multiple runs) and caught real bugs:
- macOS-only compile errors in `network.rs` (type inference on the macos ifconfig path) +
  dead-code warnings in `cpu.rs`/`cpuusage.rs`/`cpucache.rs` — fixed with explicit types
  and `#[cfg(target_os = "linux")]` gating (never compiled on Linux, so local CI missed them).
- `mlugg/setup-zig@v1` requests `zig-linux-x86_64-*` (os-arch) but ziglang.org serves
  `zig-x86_64-linux-*` (arch-os) — every mirror 404s, so the 3 musl Linux jobs failed at
  the toolchain step (not the code). Replaced with a pinned direct curl download of
  `zig-x86_64-linux-0.16.0.tar.xz` (runners are always x86_64, so one tarball cross-links
  all three musl targets).
- `softprops/action-gh-release` requires a tag, so tagless `workflow_dispatch` runs red at
  the final upload step; gated with `if: startsWith(github.ref, 'refs/tags/v')`.
Post-fix validation status: macOS ✓ (aarch64 + x86_64), Linux pending re-run of the Zig fix.

---

## Phase 1 — Visual overhaul

### 1.1 Ratatui layout engine — 🚫 rejected
Ratatui is for *interactive* TUI apps that own the terminal (raw mode, redraw loops); a
one-shot fetch tool doesn't fit that model. More importantly, it would replace the **Tera
template system** — one of the three headline features in the README comparison table.
Output stays template-driven (`flexfetch-core/src/template.rs`). (See 2.1 for the one place
ratatui IS appropriate.)

### 1.2 True color, gradients, Nerd Fonts — ✅ (pre-existing)
Already implemented: 27 theme presets (plus a `none` fallback), per-char gradient titles,
17 configurable Nerd Font icons, palette styles (`flexfetch-core/src/theme.rs`, `config.rs`).
The plan's OSC-4
auto-theme detection and `assets/themes/*.toml` externalization were not adopted — themes
are embedded as consts (self-contained binary, no runtime file reads).

### 1.3 Dynamic logo system — ✅ (pre-existing, exceeds plan)
Already implemented: 527+ fastfetch ASCII logos + custom high-quality logos, PNG image
logos rendered via Kitty / iTerm2 / Sixel / block-character protocols with terminal
detection, `{cN}` color-token injection, small-logo fallbacks, user override via
`~/.config/flexfetch/logos/` (`logo.rs`, `image_logo.rs`, `fastfetch_logos.rs`).

---

## Phase 2 — Killer features

### 2.1 Live dashboard TUI (`--live`) — ✅ done
Real-time monitor (sparklines, gauges, top processes) — the one genuinely new headline
feature. **This is where ratatui + crossterm belong** (an interactive mode, not the
one-shot renderer). Watch mode exists but only reprints the static view.
- Implemented: `flexfetch-cli/src/live.rs` — ratatui 0.30 + crossterm 0.29 behind a
  `live` feature (default on). CPU gauge + 60-sample sparkline (delta of `/proc/stat`),
  memory gauge + sparkline (reuses the `memory` collector via `run_individual`),
  top-10 processes by CPU (custom `/proc/<pid>/stat` + `statm` parsing, delta-based
  CPU%), network RX/TX rates (`/sys/class/net` counters, real elapsed time). `q`/`Esc`
  quit, `Space` forces a refresh. Pure-Rust deps, so musl cross-builds are unaffected.
- Size impact: native 6.29 MB → 6.50 MB (+0.2 MB) with `--features live`; still well
  under the 8 MB release gate. `--no-default-features` drops ratatui/crossterm entirely.
- Release binaries ship the dashboard (`--no-default-features --features live`).
- Notes: first sample has no process baseline (deltas need two samples); `proc_prev`
  grows by pid churn (bounded per run, acceptable for v1); CPU% per proc is % of one
  core (multi-core procs can exceed 100%, like top/htop).

### 2.2 Screenshot & export pipeline — ✅ (pre-existing, exceeds plan)
Already implemented: `--export svg|html|png|markdown` + `-f json`, ANSI→spans→SVG/HTML/PNG
rendering, markdown stripping (`flexfetch-core/src/export.rs`). The plan's resvg/headless
browser pipeline and 0x0.st upload were not adopted (unnecessary weight; no `upload` feature).

### 2.3 Context-aware "smart fetch" (`--smart`) — ✅ done
Show context-relevant info based on `$PWD`:
- Implemented: three new modules, all pure std (no new deps):
  - `git` (`modules/git.rs`) — branch, ahead/behind vs upstream, dirty file count via
    `git` CLI (no `git2` C dep); empty map outside a repo so the line is omitted.
  - `project` (`modules/project.rs`) — walks up from `$PWD` detecting manifests
    (Cargo.toml/package.json/go.mod/pyproject.toml/requirements.txt/pom.xml/composer.json/
    Gemfile/mix.exs/build.gradle/CMakeLists.txt/Makefile/Dockerfile/docker-compose.yml)
    → `Project: Rust — flexfetch`.
  - `context` (`modules/context.rs`) — container (`/.dockerenv`, `/run/.containerenv`,
    `/proc/1/cgroup`), venv (`VIRTUAL_ENV`), SSH session (`SSH_CLIENT`/`SSH_CONNECTION`).
- CLI: `--smart` appends git/project/context to the module selection (dedup'd, works with
  any preset/`--modules`). Template sections added to `templates/default.tera`.
- Tests: 4 new unit tests (git non-repo → empty map with cwd guard; project cargo/node/none)
  + integration test covers all three; 32/32 pass, clippy/fmt clean on all feature configs.

### 2.4 System health score & micro-benchmark — ✅ done
- `health` module (`modules/health.rs`): score 0–100, deducts for disk >90%,
  swap >50%, load >1/core, battery <80%; emits score + grade (Excellent/Good/
  Fair/Poor) + contributing notes. Pure std. `--health` appends it (dedup'd);
  `health` also works as a regular module/preset member. Template line added.
- `--benchmark` extended: `--benchmark` keeps per-module timing; `--benchmark N`
  runs the full pipeline N times and reports min/avg for run_selected + render.

### 2.5 Remote fetch (`--ssh <host>`) — ✅ done
- `--ssh host1 --ssh host2` (repeatable): runs `ssh host flexfetch --format json`
  per host in parallel (scoped threads, order preserved), parses via the new
  `SystemInfo::from_json` (JSON → InfoValue, with a round-trip unit test), and
  renders locally. Shell-noise guard slices between first `{` and last `}`.
- Fallback: if the remote lacks flexfetch, scp the current binary to
  `/tmp/flexfetch-<pid>` and run it there. `--pipe` is deliberately not passed
  (stdout over ssh is already a pipe; keeps older remote versions working).

### 2.6 Music player (MPRIS2) — ✅ done
- `music` feature (opt-in, `flexfetch-core` + `-cli`): `zbus` 5
  (`blocking-api` + `async-io`, pure Rust) queries MPRIS directly — ListNames,
  `Properties.Get` Metadata (`a{sv}` → HashMap) + PlaybackStatus. `dbus-send`
  shell-out remains as the fallback when zbus fails or the feature is off.
  macOS `nowplaying-cli` path unchanged. Not in default features (keeps the
  diet; release builds unaffected).

---

## Phase 3 — Polish & distribution

### 3.1 Interactive config wizard (`--wizard`) — ✅ done
- Ratatui 4-step wizard (`flexfetch-cli/src/wizard.rs`, behind the `live`
  feature): 1) module checklist (`space` toggle, `a` all, defaults = default
  preset), 2) theme picker with live preview (theme::resolve + gradient),
  3) layout (box style + frame), 4) save summary → writes
  `~/.config/flexfetch/config.toml`. `q`/`Esc` always cancel (even on the save
  step); `y`/`enter` confirms. Hot-reload via `notify` deferred (not needed —
  config is read once per run).

### 3.2 Shell integration — ✅ done
- Completions: ✅ bash/zsh/fish already shipped (`completions/`).
- `--prompt` single-line ANSI-free mode (`cachyos | CPU 12% | RAM 3.2 GiB/15.3 GiB`)
  for shell prompts (`PS1="$(flexfetch --prompt) $ "`) — done.
- `--motd` plain-text mode (normal output with ANSI stripped) for `/etc/motd` — done.
- `clap_complete` generator subcommand — ✅ done: `flexfetch completions bash|zsh|fish`
  behind a `completions` feature (default on, ships in release binaries); completion
  files in `completions/` are now regenerated from the generator.

### 3.3 Documentation & branding — ✅ done
- ✅ README (hero images, install, themes, plugins, templates, comparison table,
  new sections for `--smart`/`--health`/`--prompt`/`--motd`/`--ssh`/`--wizard`,
  completions regeneration, diet sizes), man page (`doc/flexfetch.1` updated with
  all new flags + examples), `install.sh` (OS/arch aware since 0.3).
- ✅ mdBook docs site + GitHub Pages: `book/` (11 chapters: intro, install, quick
  start, config, modules, templates, plugins, themes, shell integration, features,
  output, building, FAQ) + `.github/workflows/docs.yml` (build + deploy via
  `actions/deploy-pages`, publishes to the `gh-pages` environment).
- ⬜ Optional (not started): `clap_mangen` to regenerate the man page from clap
  derive definitions.

### 3.4 Hot-reload (config changes) — ✅ done
`--watch` and `--live` now detect config-file changes by **mtime** (no `notify`
dependency): the file is stat'ed each refresh and, on change, config/custom modules
are re-read and re-applied. `--live` shows a "config reloaded" notice in the header.
This replaces the earlier "deferred — not needed" note in 3.1.

---

## Phase 4 — Domination (stored Aug 2026, not yet started)

> Goal: make flexfetch objectively superior to fastfetch/macchina/hyfetch/neofetch in
> every measurable dimension (latency, size, features, visuals) while keeping every
> advanced feature behind a compile-time gate so the minimal binary stays sub-1 MB.
> All tasks ⬜ pending unless marked otherwise. The full original plan text is in the
> chat history; this section is the canonical index + status tracker.

### Pillar A — Zero-cost performance

| Task | What | Feature gate | Status |
| ---- | ---- | ------------ | ------ |
| 4.1 | **Sub-10 ms cold start**: eliminate process-spawning collectors, mmap'd `/proc`, `phf` map, lazy zstd assets, `hyperfine` CI gate | `parallel`, `fast-paths` (default) | ✅ (Aug 2026) |
| 4.2 | **Lock-free live dashboard**: crossbeam overwrite channel (bounded(1)) collector→renderer, `repr(C)` snapshot with atomics, pre-allocated sparkline ring buffer, 16 ms render budget, core affinity | `live` (exists), `lockfree` | ⬜ |
| 4.3 | **SIMD benchmark/processing**: AVX2/NEON vectorized CPU bench, `libc::memset` memory bandwidth bench, SIMD sparkline min/max, SIMD logo gradient | `simd` (default x86_64) | ⬜ |

### Pillar B — Deep system introspection

| Task | What | Feature gate | Status |
| ---- | ---- | ------------ | ------ |
| 4.4 | **eBPF metrics**: power via RAPL (libbpf-rs), disk I/O latency histograms, TCP retransmits, syscall rate. Privilege-separated `flexfetch-bpf` (setcap cap_bpf+eip) talking over UDS; graceful `/proc` fallback | `bpf` (off) | ⬜ |
| 4.5 | **GPU deep inspection**: NVML (nvml-wrapper) for VRAM/temp/power/fan/CC/processes/ECC; AMD `/sys/class/drm` gpu_metrics; Intel gt freq; optional Vulkan via `ash` | `gpu-nvml`, `gpu-amd`, `gpu-intel`, `vulkan` | ⬜ |
| 4.6 | **Filesystem deep dive**: BTRFS profile/compression/subvols, ZFS pools, ZRAM ratio, LVM mapping, LUKS cipher, optional SMART, mount options | `fs-btrfs`, `fs-zfs`, `fs-zram`, `fs-lvm`, `fs-luks`, `fs-smart` | ⬜ |
| 4.7 | **Terminal fingerprinting**: OSC 50 font query, OSC 4/10/11 palette swatch, image protocol negotiation (kitty/iTerm/sixel), OSC 8 hyperlinks, pixel dims/DPI | `term-deep` (default) | ⬜ |
| 4.8 | **Wallpaper & desktop context**: wallpaper path per DE/WM (gsettings/KDE cfg/sway/hyprland/osascript/registry), dominant colors via color-thief, GTK/icon/cursor themes | `desktop-context` | ⬜ |

### Pillar C — Workflow & integration power

| Task | What | Feature gate | Status |
| ---- | ---- | ------------ | ------ |
| 4.9 | **Diff mode** (`--diff <host1> <host2>`): compare local/remote/JSON datasets, 3-column aligned table, semantic highlight, HTML/MD export | `diff` (default) | ⬜ |
| 4.10 | **Infrastructure exports**: `--format ansible|terraform|csv|prometheus` + `--discover` mDNS service discovery | `export-infra` (default) | ⬜ |
| 4.11 | **QR config sharing**: `--qr` renders base64+zstd config as terminal QR (unicode blocks), `--import-qr` reads via rqrr | `qr` | ✅ (Aug 2026) |
| 4.12 | **WASM plugin runtime**: wasmtime behind `wasm-plugins` (off), WIT collector contract, fuel-limited sandboxed `.wasm` plugins in `~/.config/flexfetch/plugins/` | `wasm-plugins` (off) | ⬜ |

### Pillar D — Marketing hooks

| Task | What | Feature gate | Status |
| ---- | ---- | ------------ | ------ |
| 4.13 | **Local AI summary** (`--ai-summary`): llama-cpp-rs + bundled Q4_K_M gguf (~30 MB), on-device one-liner roast | `ai` (off, heavy) | ⬜ |
| 4.14 | **Weather & geolocation**: embedded GeoLite2 + MET Norway API over a hand-rolled `TcpStream` HTTP/1.1 parser (no reqwest/hyper), 10-min cache | `weather` | ⬜ |
| 4.15 | **Container deep introspection**: docker.sock via hyperlocal, Podman, Kubernetes pod/node/limits | `container-deep` | ⬜ |

### Pillar E — Size & distribution

| Task | What | Feature gate | Status |
| ---- | ---- | ------------ | ------ |
| 4.16 | **Compile-time asset compression**: build.rs zstd blobs, `phf_codegen` perfect hash for distro→logo, lazy per-logo decompression, string dedup, panic=abort (have), system allocator | — | ⬜ |
| 4.17 | **Universal installer**: Homebrew tap, AUR `flexfetch-bin`, Nix profile, `.deb`/`.rpm` via cargo-deb/generate-rpm in CI, static musl tarballs (have), simplified install.sh | — | 🟡 (checksums + backup + .sha256 done; taps/pkgs pending) |

### Dependency graph & execution

```
Phase 0-3 (done) ──► Pillar A (4.1→4.2→4.3) ──► Pillar B (4.4-4.8)
                        └────────────────────► Pillar C (4.9-4.12) ──► Pillar D (4.13-4.15)
                                              └────────────────────► Pillar E (4.16-4.17)
```

Suggested order: **Week 1** 4.1 (foundation — forces zero-alloc collectors) · **Week 2** 4.2+4.3 ·
**Week 3** 4.5+4.6+4.8 · **Week 4** 4.4+4.7 · **Week 5** 4.9+4.10+4.14 · **Week 6** 4.11+4.12+4.15 ·
**Week 7** 4.13+4.16+4.17 · **Week 8** v2.0 release.

**Start here: Task 4.1** (sub-10 ms guarantee) — it forces zero-allocation collectors,
which feeds the live dashboard and the whole premium feel.

### Task 4.1 — zero-spawn collectors + hyperfine gate — ✅ done (Aug 2026)

Goal: eliminate `Command` spawns from every default-path collector so cold start is
bounded by file reads + network latency, not process forking. Measured on the dev box
(CachyOS, i5-12450H): **cold start 5.5 s → 686 ms** (fresh) / **242 ms** (with the
publicip cache warm), and the single worst module (**wifi** was 4.1 s) is now **24 ms**.

The remaining 686 ms is *network latency*, not spawn cost: publicip ~500 ms (api.ipify.org
round-trip, cached for 60 s so repeat runs are ~0), cpuusage 30 ms (sampling window),
battery 65 ms (sysfs + charging math).

Landed (all Linux default-path modules, macOS paths preserved + cross-check clean):
- **kernel** (`kernel.rs`) — reads `/proc/sys/kernel/ostype`+`osrelease` instead of
  `uname` (output order verified identical to `uname -srm`); macOS keeps `sysctl`.
- **packages** (`packages.rs`) — parses `/var/lib/dpkg/status` (only
  `Status: install ok installed`), `/var/lib/pacman/local/` (directory entries only —
  `ALPM_DB_VERSION`/`*_NOTICE` files were being counted!), flatpak + snap dirs; rpm
  keeps its CLI (Berkeley DB is not parseable as plain files); full CLI fallback still
  runs when no DB is readable, rayon-parallel under the `parallel` feature.
- **wm** (`wm.rs`) — WM name from env vars first, `gsettings current-wm` fallback
  (GNOME keeps the real WM, e.g. mutter, in dconf only); theme/icons/cursor/font read
  from `~/.config/gtk-{3,4}.0/settings.ini` (each file read once) with a gsettings
  fallback when no config file exists.
- **disk** (`disk.rs`) + **health** (`health.rs`) — `libc::statvfs` instead of `df`
  (`f_frsize` used for byte math; % matches `df`'s `(blocks−avail)/blocks`); disk
  parses `/proc/mounts` for real roots (ext/btrfs/xfs/zfs/f2fs/overlay — containers
  now show `/` instead of an empty list).
- **network** (`network.rs`) — `libc::getifaddrs` + sysfs MAC instead of `ip`/`cat`
  (paired with `freeifaddrs`); macOS keeps `ifconfig`.
- **wifi** (`wifi.rs`) — `nmcli … dev wifi list --rescan no` (space-separated! `=no` is
  rejected) uses NetworkManager's *cached* scan: 24 ms vs 4.17 s. Caveat: on a fresh
  boot with no scan yet it may report "not connected".
- **publicip** (`publicip.rs`) — raw `TcpStream` HTTP/1.1 GET to api.ipify.org:80 (no
  `curl` spawn; also faster: 495 ms vs curl's 715 ms) + shared-cache (60 s TTL) so
  repeated runs skip the round-trip entirely.
- **cpuusage** (`cpuusage.rs`) — sampling window 100 ms → 30 ms (removed dead
  `_start`/`Instant`); still a stable aggregate-cpu delta.
- **--benchmark** (`main.rs`) — now reports `cold start` measured from process entry
  (before clap parse), plus per-module timings, `run_selected` avg/min, render avg/min.

Validation: 34/34 tests, clippy `-D warnings` clean, fmt clean, macOS core + cli
cross-checks clean, all feature configs build (default / minimal / release config).

Deferred optimizations (not required for the sub-10 ms target — they feed 4.16
asset compression instead): mmap'd `/proc` parsing (memmap2/nom zero-copy),
`phf::Map` for the distro→logo map, `&'static str` label interning, lazy zstd
logo decompression.

### Task 4.11 — QR config sharing — ✅ done (Aug 2026)
`--qr` encodes the current config as a base64+zstd blob and renders it as a terminal
QR (unicode blocks, `qrcode` crate); `--import-qr <image>` decodes a screenshot of it
back into the config path (existing file backed up, `rqrr` decode, `image` png-only
for the reader). Whole feature gated behind `qr` (opt-in — `zstd-sys` is a C dep, so
it stays out of the pure-Rust musl release pipeline and the minimal binary). CI
coverage: the `qr-feature` job runs tests + clippy with `--features qr`.

### Task 4.1 (remainder) — hyperfine cold-start CI gate — ✅ done (Aug 2026)
`ci.yml` gained a `perf-gate` job: builds the minimal release
(`--no-default-features`), `cargo install hyperfine`, measures
`./target/release/flexfetch --minimal` (title/separator/os/kernel/uptime — all
zero-spawn file reads) with `--warmup 5 --runs 50`, and fails if the mean exceeds
10 ms (python3 parses the `--export-json` output; raise `limit` to 0.015 if shared
runners flake rather than deleting the gate).

### Task 4.17 (installer) — checksums + backup + release wiring — 🟡 (Aug 2026)
- `install.sh`: SHA-256 verification (fail-closed on mismatch, warn+skip only when no
  sha tool exists or the checksum file is unfetchable; `sha256sum` with a
  `shasum -a 256` fallback for macOS), timestamped `.bak` backup of the existing
  binary before overwrite, temp-dir cleanup on both success and failure paths.
- `release.yml`: a "Generate checksums" step emits `<artifact>.sha256` per tarball
  (sha256sum / shasum -a 256 detection for the macOS runner), and the release upload
  now includes tarballs + checksums + `install.sh` — so
  `curl …/releases/latest/download/install.sh | sh` works and every download is
  verifiable.
- Still pending: Homebrew tap, AUR `flexfetch-bin`, Nix profile, `.deb`/`.rpm` in CI.

---

## Phase 5 — Ecosystem (stored Aug 2026, not yet started)

> Goal: turn flexfetch from a tool into an **ecosystem** — distribution channels,
> intelligence, and community extension points. All tasks ⬜ pending unless marked
> otherwise. The full original plan text is in the chat history; this section is the
> canonical index + status tracker.

### Pillar F — Distribution & lock-in

| Task | What | Status |
| ---- | ---- | ------ |
| 5.1 | **Nix flake + Home Manager module**: `flake.nix` (rustPlatform.buildRustPackage, cargoLock) + `homeManagerModules.default` writing `~/.config/flexfetch/config.toml` from `programs.flexfetch.settings` | 🟡 |
| 5.2 | **GitHub Action (`flexfetch-action`)**: composite action running flexfetch in CI with a `github` output format (::group:: annotations); separate marketplace repo | ✅ |
| 5.3 | **Tmux integration**: `--tmux-config` snippet + tiny `flexfetch-tmux` helper binary showing the fetch in idle panes | ✅ |

### Pillar G — Intelligence & context

| Task | What | Feature gate | Status |
| ---- | ---- | ------------ | ------ |
| 5.4 | **Wallpaper auto-theming** (`--auto-theme`): color-thief dominant colors → on-the-fly theme, cached to `/tmp` by wallpaper mtime | `auto-theme` (image) | ⬜ |
| 5.5 | **SQLite metrics history** (`history.db`): snapshots table, `--history-graph cpu|memory --hours N` ASCII sparkline, `--history-export csv`, 90-day prune | `history` (rusqlite) | ⬜ |
| 5.6 | **Critical health notifications** (`--daemon`): notify-rust/mac-notification-sys on threshold breach (cpu/mem/disk/temp), 60 s poll | `notifications` | ⬜ |

### Pillar H — Community & extensibility

| Task | What | Status |
| ---- | ---- | ------ |
| 5.7 | **Plugin registry**: `flexfetch plugin search|install|list|update` against a hosted `registry.toml` with checksum + min-version checks | ⬜ |
| 5.8 | **Crowdsourced hardware DB**: compressed JSON on GitHub Pages (`hardware.json.zst`, ~50 KB), `--update-db`, cache + offline hex fallback | ✅ |
| 5.9 | **AUR PKGBUILD + Homebrew tap**: native package-manager installs | 🟡 |

### Pillar I — Unfair advantages

| Task | What | Status |
| ---- | ---- | ------ |
| 5.10 | **ASCII cinema** (`--live --record session.cast`): asciinema v2-format recording of the live dashboard | ⬜ |
| 5.11 | **Container image**: static musl → scratch `~1 MB` image, GHCR publishing in CI (`ghcr.io/mahesh-diwan/flexfetch`) | ⬜ |

### Execution priority

| Week | Focus | Tasks |
| ---- | ----- | ----- |
| 9 | Distribution | 5.1 (Nix), 5.9 (AUR/Homebrew), 5.11 (Container) |
| 10 | Integration | 5.2 (GitHub Action), 5.3 (Tmux), 5.8 (HW DB) | ✅ done |
| 11 | Intelligence | 5.4 (Auto-theme), 5.5 (History), 5.6 (Notifications) |
| 12 | Community | 5.7 (Plugin Registry), 5.10 (ASCII Cinema) |

**Immediate next step: Week 11 — Intelligence: 5.4 (Auto-theme), 5.5 (History), 5.6 (Notifications)** —
Week 9 (distribution: Nix flake, AUR/Homebrew, container) and Week 10 (integration:
GitHub Action, tmux, hardware DB) are both shipped.

### Task 5.2 — GitHub Action (`flexfetch-action`) — ✅ (Aug 2026)
- `export_github()` in `flexfetch-core/src/export.rs`: renders a collapsible
  `::group::` block with `[36m`-colorized `{:<14}` keys (skips title/separator,
  drops empty values) — GitHub shows it as a foldable, colorized section in any
  workflow step's log.
- `--format github` wired into both `main.rs` render paths (direct + export).
- `packaging/flexfetch-action/action.yml`: composite action (installs flexfetch
  when missing via install.sh, then runs `flexfetch --format github`, honoring
  optional `theme`/`modules` inputs). Marketplace publish is a separate repo
  action (`mahesh-diwan/flexfetch-action`), like the AUR tap.

### Task 5.3 — Tmux integration — ✅ (Aug 2026)
- `flexfetch --tmux-config` (`tools.rs::print_tmux_config`): prints a
  `~/.tmux.conf` snippet — `run-shell ~/.local/bin/flexfetch-tmux` fires in every
  new pane, and the helper only shows the fetch when the pane is idle (its
  current command is a shell).
- `flexfetch-cli/src/bin/flexfetch-tmux.rs`: a second `[[bin]]` (pure std, no
  deps — builds in every feature config) that reads `$TMUX_PANE`, checks
  `tmux list-panes -F '#{pane_id} #{pane_current_command}'` against
  bash/zsh/fish/sh/nu, and prints a compact `--minimal` fetch. install.sh places
  it next to the main binary.

### Task 5.8 — Crowdsourced hardware DB — ✅ (Aug 2026)
- `flexfetch-core/src/hardware_db.rs` (pure std, no deps): parses a flat
  `{ "pci": { "10de:2684": "NVIDIA GeForce RTX 4090" }, "usb": ... }` JSON
  (seed bundled via `include_str!`; cached copy refreshed from the repo raw URL
  — `FLEXFETCH_HWDB_URL` overridable). `lookup(vendor, device)` normalizes
  `0x`/case, checks the cache then the seed, returns `None` for misses so
  callers fall back to raw hex/driver names.
- `--update-db` (main.rs early handler): `hardware_db::refresh()` downloads the
  latest DB into the cache dir via curl (consistent with `--update`/install.sh),
  validates the payload has entries, fails with a clear error otherwise.
- `gpu.rs` integration: for each `/sys/class/drm/card*` entry, reads
  `device/vendor` + `device/device` and resolves the friendly model name from
  the DB (dedup'd), falling back to the driver name.
- Seed data: `flexfetch-core/data/hardware.json` (26 GPU ids across
  NVIDIA/AMD/Intel + 6 USB ids).

### Task 5.1 — Nix flake + Home Manager module — 🟡 (Aug 2026, first batch)
`flake.nix` at repo root: `rustPlatform.buildRustPackage` with `cargoLock.lockFile =
./Cargo.lock`, version pulled from `Cargo.toml`'s `[workspace.package]`; exposes
`packages.default` (release config: `--no-default-features --features
live,image-logos,completions`), `packages.full` (default features, incl. Lua) and
`packages.minimal`; plus a single system-agnostic `homeManagerModules.default`
writing the user config from `programs.flexfetch.settings` via
`lib.generators.toTOML`. CI validates with `DeterminateSystems/nix-installer-action`
(`nix build .#default` + `.#minimal` + `nix flake check`).

Pending: **generate + commit `flake.lock`** (no nix on the dev box, so CI generates
it on the fly — builds aren't pinned until it's committed; run `nix flake lock` on
a nix machine once).

### Task 5.9 — AUR PKGBUILD + Homebrew tap — 🟡 (Aug 2026, first batch)
`packaging/PKGBUILD` (arch: x86_64 aarch64; source tarball checksum pinned;
install: binary + man page + completions — note: **no** `assets/themes`/`assets/logos`
dirs exist, themes are embedded consts, so those install lines from the plan were
dropped) and `packaging/flexfetch.rb` (Homebrew formula: `cargo build` the release
config, install bin + man + completions). Publishing to AUR (`git push` to
aur.archlinux.org) and the `homebrew-flexfetch` tap repo are separate repo actions;
the files ship in-repo so the packages can be cut from any release.

### Task 5.11 — Container image + GHCR — 🟡 (Aug 2026, first batch)
`Dockerfile` builds the **minimal** static musl binary (`--no-default-features`,
no TUI needed in a container) into a `scratch` image; `release.yml` gains a `docker`
job (tag-push gated) that builds + pushes `ghcr.io/mahesh-diwan/flexfetch:{tag}`
and `:latest` on tag pushes, multi-arch amd64+arm64 via QEMU. Locally validated:
builds + runs, 2.67 MB image. Usage:
`docker run --rm -it ghcr.io/mahesh-diwan/flexfetch`. Note: the arm64 variant
compiles the Rust tree under QEMU emulation on the release job (slower); if release
latency matters, cross-compile with cargo-zigbuild in the Dockerfile instead.

---

---

## Phase 6 — Visual overhaul: "System Log → System Art" (stored Aug 2026)

> Goal: kill the monochrome, cramped, tree-lined output. All tasks ⬜ unless
> marked done. The pasted plan targeted a ratatui/`crates/ff-renderer` codebase
> that does not exist here — flexfetch's reality is `templates/default.tera` +
> `template.rs` (Tera + plain fallback), embedded const themes, `${N}` logos.
> Every task below is the reality-adapted equivalent.

| Task | Plan's version | Reality-adapted in flexfetch | Status |
| ---- | -------------- | ---------------------------- | ------ |
| 6.1 | Kill tree lines → `Key • Value` rows | default.tera rows rewritten: no `├─`/`╰─`, keys padded to `display.key_width` via a new `pad` Tera filter, separator from `display.separator` | ✅ |
| 6.2 | Theme colors on rows | Rows now wrap key/sep/value in `theme_keys`/`theme_sep`/`theme_values` (previously only the title was colored); default theme is now `catppuccin` so output has color out of the box | ✅ |
| 6.3 | Merge/dedup collectors | `show_wm` (hide WM when DE==WM) + `show_resolution` (hide Resolution when Display reports it) computed in `template.rs` and honored by both Tera + plain renderers; also surfaces `de`/`wm`/`packages`/`shell`/`colors` modules that were silently filtered out of Tera runs | ✅ |
| 6.4 | Bigger, color-injected logo | `logo::detect` now prefers the **larger** of custom vs fastfetch logo (e.g. CachyOS 10→25 lines); adds a OnceLock cache so `make_logo`'s `Box::leak` stops leaking per `--live` refresh | ✅ |
| 6.5 | Logo vertically centered | `TeraEngine::render` splits empty padding above/below the art instead of all-at-bottom | ✅ |
| 6.6 | Color swatch row | `colors` module now renders as an inline `██` palette row (Tera `palette_display` filter + plain-renderer swatch builder) instead of being dropped | ✅ |
| 6.7 | Adaptive width (compact < 80 cols) | `render` reads `$COLUMNS`; when < 80 the logo is skipped so rows never wrap | ✅ |

**Note:** the plan's `%%` double-percent bug does not exist in flexfetch (memory/disk
emit single `%`).

---

## Rejected / decisions (do not re-propose without new justification)

| Idea | Why rejected |
| ---- | ------------ |
| `crates/ff-*` workspace rename | Pure churn: breaks every import, CI, install script, tests for zero gain. Workspace already clean. |
| Ratatui for the one-shot renderer | Wrong tool (interactive-only model) and would kill the Tera-template differentiator. |
| Async collectors + tokio + async_trait | Synchronous + Rayon is faster to land and simpler; tokio runtime is overkill for a one-shot CLI. |
| `InfoItem { label, value }` redesign | Regresses the `InfoValue` enum (Scalar/Map/List/Table) that powers template context and JSON. |
| `figment` config | Layered TOML merging already exists; figment only adds env layering (nice-to-have, not a rewrite). |
| `inventory`/`linkme` registry | Unnecessary; `OnceLock` registry is already static + parallel. |
| `sysinfo` crate | Not used at all; custom `/proc` parsers are fine and lighter. |
| `reqwest` → `ureq`/hyper HTTP | No HTTP in the core; `publicip` uses `curl` externally (could become `ureq` later if wanted). |
| `git2` for git context | Heavy C dep; shell out to `git` instead. |
| External `assets/themes/*.toml` | Themes embedded as consts — self-contained binary, no runtime reads. |
| resvg/headless-chrome export pipeline | Existing ANSI→SVG/HTML/PNG export is lighter and works. |

## Next actions (priority order)

1. **Add a CI test workflow — ✅ done (Aug 2026)** — `ci.yml` runs on push to main + PRs:
   `test` (`cargo test --workspace`), `clippy` (`--all-targets -D warnings`, strict),
   `fmt` (`cargo fmt --all -- --check`), and `minimal-build` (guards the documented
   `--no-default-features` path). Shared cargo cache (registry+git only), concurrency
   cancel-in-progress. Also fixed 5 pre-existing clippy lints that would have red the
   gate (needless `&` in `logo.rs`, unused `use super::*` in `lib.rs`, 3× `len > 0` in
   `logo_tests.rs`).
2. **Validate the release matrix on GitHub — ✅ done (Aug 2026)** — `workflow_dispatch`
   proved the macOS aarch64 + x86_64 cross-builds (green) and caught + fixed 3 real
   issues (macOS-only compile bugs, the setup-zig naming bug, the tagless-upload red).
3. **2.1 Live dashboard (`--live`)** — the flagship new feature; ratatui/crossterm,
   reuse existing collectors.
4. **Finish the diet — ✅ done (Aug 2026)** — `image` gated behind `image-logos`
   (default on); tera + rayon gated behind `tera`/`parallel` (default on).
   `--no-default-features`: minimal **1.53 MB** (< 3 MB target met); release pipeline
   (live+image-logos+completions) 2.02 MB; full default 6.33 MB. Release size gate
   tightened 8 MB → 4 MB. CI guards the feature-off path (minimal-build job builds
   + runs core tests + clippy with `--no-default-features`).
5. **Remaining Phase 2/3 items — ✅ done (Aug 2026)** — the whole backlog landed in
   one batch: 2.4 health score + `--benchmark N` micro-benchmark, 2.5 `--ssh`
   remote fetch (parallel, scp fallback, `SystemInfo::from_json`), 2.6 `music`
   feature (zbus MPRIS), 3.1 `--wizard` (ratatui config wizard), 3.2 `--prompt`
   + `--motd` + `clap_complete` generator, 3.3 docs (README + man page + mdBook
   site + GitHub Pages), 3.4 mtime hot-reload in `--watch`/`--live`. All validated:
   34/34 tests, clippy clean on default/minimal/music configs, fmt clean.
6. Remaining optional/deferred: `clap_mangen` man-page regeneration, `notify`-
   based hot-reload (mtime works, no dep needed), and finishing the release-matrix
   validation (macOS ✓; Linux musl jobs were fixed via direct zig download — final
   re-run pending).
7. **Phase 4 — Domination — 🟡 Task 4.1 done, 4.11 done, 4.17 partial (Aug 2026)** —
   first batch landed: every default-path collector is zero-spawn
   (kernel/packages/wm/disk/health/network/wifi/publicip/cpuusage) + `--benchmark`
   cold-start reporting. Cold start 5.5 s → 686 ms (242 ms with publicip cache);
   wifi 4.1 s → 24 ms. Then: hyperfine `perf-gate` CI job (minimal build, mean <
   10 ms) landed; QR config sharing (`--qr`/`--import-qr`, `qr` feature) landed;
   installer hardened (checksums + .bak backup + `.sha256` artifacts in
   release.yml); `--update`/`--doctor`/`--hook` shell-integration commands landed
   (`flexfetch-cli/src/tools.rs`); git-cliff changelog config (`cliff.toml`).
   Next: 4.2 lock-free live dashboard, 4.3 SIMD, then Pillar B.

## Reference

- Original plan source: pasted v2.0 plan (Rust-native optimization plan) + pasted
  Phase 4 "Domination" plan (Aug 2026) — the canonical Phase 4 index is above.
- Design docs: `docs/superpowers/specs/`, `docs/superpowers/plans/`, `docs/superpowers/research/`.
- Size baselines: native 6.33 MB (full default) · release pipeline (live+image-logos+
  completions, no tera/rayon) 2.09 MB · minimal (--no-default-features) 1.53 MB.
  Phase 4.16 targets a sub-1 MB minimal via asset compression + perfect hashing.
