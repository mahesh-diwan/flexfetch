# Flexfetch Research: Landscape, Features, and Performance Roadmap

_Research date: Aug 2026 · flexfetch v1.0.2 · sources cited inline_

---

## Part 1 — The system-info tool landscape (2024–2026)

### Neofetch: the dead king

Bash, ~11,500 LOC single script, 23.7k stars. Everything detected by spawning
subprocesses (`uname`, `xprop`, `pacman`, …). Last release 7.1.0 in **2020**;
archived **Apr 2024** ("have taken up farming" — Dylan Araps left software).
Bug reports piled up for years; newer CPUs/GPUs misreport to this day.

**Legacy:** it defined the genre's UX contract — logo left, key/value block
right, color blocks bottom, `${c1}`..`${c6}` ASCII color placeholders — that
every successor still implements.

Sources: [OMG! Ubuntu](https://www.omgubuntu.co.uk/2024/04/neofetch-system-info-tool-is-dead), [Ars Technica](https://arstechnica.com/gadgets/2024/09/neofetch-is-over-but-many-screenshot-system-info-tools-stand-ready/)

### Hyfetch — de-facto neofetch successor

Fork adding pride-flag recoloring (~100 presets); ships `neowofetch`, an
updated neofetch backend. Very active: v2.x is migrating Python → Rust wrapper,
and is **phasing out its own backend in favor of fastfetch** ("time needed to
maintain the NF backend exceeds our capacity"). Pluggable backends: neowofetch,
fastfetch, macchina, qwqfetch.

**Ecosystem lesson:** fastfetch 2.50.x broke hyfetch's fastfetch backend by
removing `--os-key` ([hyfetch#418](https://github.com/hykilpikonna/hyfetch/issues/418))
— downstream tools coupling to another CLI's flags is brittle. Flexfetch's
own JSON export should be versioned and stable.

### screenFetch — end of life

The original (2009). Bash, 6,800 lines. Maintainer's farewell (Dec 2024):
rewrite attempt abandoned, v3.9.9 "likely the last release"
([discussion #805](https://github.com/KittyKatt/screenFetch/discussions/805)).

### pfetch / ufetch — the minimal pole

- **pfetch**: POSIX sh, config via env vars (`PF_INFO`, `PF_ASCII`), instant
  startup vs neofetch's visible pause. Minimalism as pedagogy. Unmaintained
  since ~2020 but still works.
- **ufetch**: one ~30-line sh script per distro. The extreme minimal end.

Lesson: there is durable demand for a sub-millisecond, zero-config fetch.

### freshfetch — the cautionary tale

Rust frontend that shelled out to fastfetch's JSON output, rendered
neofetch-style with Lua customization. Stuck "beta" forever, dead ~2024.
Lessons: (a) wrapping another tool's CLI adds latency and breaks on upstream
changes; (b) scripting-language plugin systems are over-engineering for a
fetch tool (flexfetch removed its Lua plugin system in v0.30 — validated);
(c) "beta forever" kills projects faster than archiving.

### 2025–2026 newcomers (all racing on milliseconds)

| Tool                                                                  | Claim                                               | Notable                                                                                              |
| --------------------------------------------------------------------- | --------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| [fetchx](https://github.com/swadhinbiswas/fetchx) (Mar 2026)          | 10–50ms                                             | 8-thread parallel detection, 5 image backends, **JSON output + daemon mode for Waybar**, TOML config |
| [hyperfetch](https://github.com/revanthnemtoor/hyperfetch) (Mar 2026) | **1.8ms mean** (4.6× faster than fastfetch's 8.3ms) | live TUI, hardware caching                                                                           |
| leenfetch                                                             | —                                                   | modular layout tags, JSONC config, JSON output + SSH remote fetch                                    |
| NerdFetch                                                             | —                                                   | POSIX sh + Nerd Font glyphs instead of ASCII art, active                                             |
| macchina                                                              | —                                                   | Rust, libmacchina; **officially in maintenance mode**                                                |

The ms race is real: hyperfetch publicly benchmarks against fastfetch with
hyperfine. Flexfetch's `--flash` mode (~3ms measured locally, below) is
already competitive; the marketing of speed is not.

---

## Part 2 — Fastfetch deep study

### Architecture

Pure C (C23 as of 2.67), CMake, near-zero hard deps. Optional libs
(libchafa, ImageMagick, zlib, Vulkan, Wayland, xcb, dbus, sqlite3) are
**dlopen'd at runtime** via `FF_LIBRARY_LOAD` — `--list-features` shows what
was compiled in. Embedded yyjson for JSON.

Three layers:

1. **Module layer** (`src/modules/`) — compile-time registry
   `ffModuleInfos[]`: A–Z array indexed by first letter; each module is a
   polymorphic interface (`initOptions`, `parseJsonObject`, `printModule`,
   `generateJsonResult`, `formatArgs`).
2. **Detection interface** (`src/detection/`) — platform-agnostic signatures
   (`ffDetectOS`).
3. **Platform implementations** — e.g. `memory_linux.c` reads `/proc/meminfo`
   directly into a fixed stack buffer: no allocation, no subprocess.

Two execution paths: registry-driven loop, plus **flashfetch** which skips
registry lookups entirely, stack-allocates options via macro, and calls
`ffPrintCPU()` etc. directly — a "completely precompiled" config.
_(Flexfetch's `render_plain` fast path mirrors this pattern.)_

### Performance profile

- ~19ms typical vs neofetch's 566ms (~30×).
- Speed comes from: direct `/proc` reads into fixed buffers, zero-allocation
  `FFstrbuf` building, lazy dlopen, no shell-out for common modules.
- **Multithreading is narrow**: `--thread` only parallelizes HTTP requests
  (PublicIP/Weather). Modules run sequentially.
- Image logos cache processed sixel/kitty/chafa output on disk.
- `--stat` prints per-module ms timings — first-class perf debugging.
- Known slow spots: xrandr displayserver detection (500–1250ms with external
  monitors, [#634](https://github.com/fastfetch-cli/fastfetch/issues/634));
  DBus timeouts on headless boxes (Bluetooth +10s on a Pi,
  [#1450](https://github.com/fastfetch-cli/fastfetch/issues/1450)).

### Feature inventory

- **70+ modules**, including exotic ones (Camera, Codec, TPM, Media, Weather).
- **16 logo types** (auto, builtin, small, file, data, sixel, kitty,
  kitty-direct, iterm, chafa, raw, none…). `kitty-direct` = terminal loads
  the file itself, fastest path. Logo position left/right/top, per-logo color
  overrides, padding controls, `media-cover` art keyword.
- **Output formats: only `default` and `json`.** No CSV/YAML/markdown/
  prometheus/svg/html. _(Flexfetch ships 9 exporters — clear differentiator.)_
- **Format strings**: named placeholders `{user-name}`, `{freq-max}`;
  truncation/padding/slicing; constants/env vars `{$VAR}`; conditional
  content; xterm-256 colors `{#@34}`; experimental embedded `lua:`/`qjs:`
  scripting inside format strings (since 2.64).
- **JSONC config** with published schema ($schema → IDE completion),
  built-in presets incl. `neofetch.jsonc` clone, 32 community examples.
- `--gen-config` interactive TUI (2.67), `-w/--watch` live refresh,
  `--processing-timeout` capping child-process/DBus waits (default 5000ms).

### Recent trajectory (2025–2026)

~monthly releases focused on **detection breadth** (new WMs, editors, package
managers, SoCs), not speed or output formats. Breaking-change churn is
significant: case-sensitive keys, removal of all per-module CLI flags (2.50),
`preRun` removed for security (2.67). Scripters are annoyed
([#610](https://github.com/fastfetch-cli/fastfetch/issues/610)).

### Weaknesses = flexfetch opportunities

1. Only 2 output formats vs flexfetch's 9 exporters.
2. Sequential module execution; slow detectors block everything.
3. Verbose default; no responsive layout / narrow-terminal auto-hide
   ([Discussion #2133](https://github.com/fastfetch-cli/fastfetch/discussions/2133)).
4. JSONC friction: broken config.jsonc breaks startup with cryptic errors;
   privacy complaints (localip/locale shown by default, #923).
5. No custom modules without forking (flexfetch has inline `custom` shell
   modules in TOML).
6. No diff mode, no cross-run caching of expensive detections.
7. Breaking-change churn — stability is a feature users will pay for.

---

## Part 3 — Measured baseline (flexfetch v1.0.2, this machine)

| Build                             | Binary size | Cold start (20 runs)      |
| --------------------------------- | ----------- | ------------------------- |
| `--release` (default features)    | **7.56 MB** | ~12.5 ms/run              |
| `--release --no-default-features` | **1.78 MB** | ~2.9 ms/run (`--minimal`) |

Already shipped: `lto=true`, `codegen-units=1`, `opt-level="z"`,
`strip=true`, `panic=abort` — the standard five-liner is done. Remaining wins
are dependency-level, not flag-level.

cargo-bloat on the minimal build (.text = 978 KB): biggest single items are
`main` (24 KB), fastfetch logo tables (21 KB), toml_edit parser (20 KB),
clap parser (19 KB), std backtrace machinery (17–12 KB), TeraEngine::render
(14 KB), serde/toml deserializers (~34 KB combined).

---

## Part 4 — Feature proposals (ranked by impact ÷ effort)

### Tier 1 — differentiators fastfetch cannot easily copy

1. **Waybar/panel JSON daemon mode** (`flexfetch --serve`). fetchx proved
   demand. Expose collected SystemInfo as JSON over a unix socket / stdout
   stream with periodic refresh; panels poll cheaply. Reuses live.rs sampler.
2. **Responsive layout** — auto-drop low-value rows when terminal height is
   tight, auto-narrow key column when width < N. Nobody does this; complaints
   about overflowing screenshots are recurring upstream.
3. **Diff mode promotion** — flexfetch already has diff; fastfetch has
   nothing like it. Document + polish (`flexfetch diff --save baseline`).
4. **Stability guarantee** — publish a policy: config keys and JSON export
   schema never break in patch/minor releases. Direct counter-positioning to
   fastfetch's churn; costs nothing.
5. **Per-module timeout budget** (like fastfetch's `--processing-timeout`)
   applied to DBus/network/custom-shell modules so a hung sensor can never
   stall the fetch.

### Tier 2 — parity where it matters

6. **`--stat` per-module timings** — trivial (wrap collect() with Instant);
   huge credibility with the ricing crowd.
7. **Image-logo disk cache** — cache rendered sixel/kitty ANSI per
   (image, size, protocol) like fastfetch does; makes image logos free after
   first run.
8. **kitty-direct logo path** — pass file path to terminal instead of
   rendering ANSI ourselves when protocol is kitty.
9. **More detection breadth** — camera, codec, player, USB devices; monthly
   cadence of "new DE/WM/editor detected" changelog entries is how fastfetch
   stays in the news.

### Tier 3 — niche / defer

10. Pride-flag theme pack (hyfetch audience) — just more themes in the table.
11. Embedded format-string scripting — we deliberately removed the plugin
    system; don't reintroduce it as Lua-in-templates.
12. WebUI config editor — fastfetch's experiment; watch, don't build.

---

## Part 5 — Snappier, faster, smaller: the plan

### Where the bytes are

Default build 7.56 MB vs minimal 1.78 MB ⇒ **5.8 MB lives behind default
features**: tera, rayon, image, zbus, qrcode/rqrr/zstd, ratatui/crossterm.
The minimal build is already excellent; the strategy is to keep shrinking the
_default_ build without cutting user-visible features.

### Size actions (ranked)

| #   | Action                                                                                                                                                          | Est. saving                              | Risk                                                  |
| --- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- | ----------------------------------------------------- |
| S1  | Gate `qr` out of default features (opt-in like `music`)                                                                                                         | ~1.5–2 MB (rqrr+zstd+qrcode+image paths) | low — flag still exists, feature-gated                |
| S2  | Replace `toml` (toml_edit-based) with `toml` 0.8's `parse` only, or `basic-toml`/`tomlrs`-class parser — we serialize once, parse often                         | ~200–400 KB                              | medium — test config round-trips                      |
| S3  | PNG decode: use `png` crate directly instead of `image` (kills generic-reader monomorphization, [image-rs#2472](https://github.com/image-rs/image/issues/2472)) | ~300–500 KB across builds                | low                                                   |
| S4  | clap → hand-rolled parser or pico-args (43 flags; osa1/tiny saved −400 KiB, firecracker measured +344 KB for clap alone)                                        | ~400–600 KB                              | high — rewrite arg surface; do last, keep completions |
| S5  | `zbus` → `dbus` crate or dlopen-at-runtime like fastfetch (zbus ≈ 25% of i3status-rs .text)                                                                     | ~500 KB–1 MB                             | medium — music/wallpaper modules                      |
| S6  | nightly `-Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort` for release artifacts only                                                   | −30–60% of remaining std bloat           | CI-only, needs nightly                                |
| S7  | PGO+BOLT in release CI (johal.in: −22% cold start, 74→47 MiB binary shrink cases)                                                                               | 10–20% startup                           | +CI minutes                                           |

Realistic endpoint: **default build ~3–4 MB, minimal build <1 MB** with S1+S3+S6.

### Startup actions

| #   | Action                                                                                                                                                   | Est. gain               |
| --- | -------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| F1  | Lazy-init everything non-essential: theme table, logo tables, Tera engine already OnceLock'd — audit remaining eager statics                             | ~0.5–1 ms               |
| F2  | Cache expensive detections (disk serial, GPU name, packages count) in `~/.cache/flexfetch/` keyed by mtime/boot-id — packages module re-counts every run | 5–50 ms on slow systems |
| F3  | Parallel collection already exists (`parallel` feature) — make it default-on for >8 modules; keep sequential under `--flash`                             | varies                  |
| F4  | Timeout budget (feature #5 above) doubles as worst-case-latency insurance                                                                                | bounded tail latency    |
| F5  | Measure with `--stat` (Tier-2 item) before optimizing further — no blind tuning                                                                          | —                       |

Note: musl static builds start ~1 ms faster than glibc dynamic (no loader
work) and our CI already ships musl targets — keep them the advertised binary.

### What NOT to do

- No no_std (loses ecosystem, saves nothing real for a CLI).
- No miniserde/nanoserde swap unless S2 profiling shows serde derive is
  actually hot — config parsing is once-per-run.
- No embedded scripting languages (freshfetch died of this).
- Don't chase hyperfetch's 1.8ms headline with unsafe tricks; our flash path
  is within noise of it already.

---

## Part 6 — Suggested sequencing

1. **v1.1**: `--stat`, per-module timeouts, responsive layout, stability
   policy doc. (All small, all visible.)
2. **v1.2**: disk cache for expensive detections + image-logo cache +
   kitty-direct. (Speed story.)
3. **v1.3**: S1 (qr opt-in) + S3 (png crate) + S6 (build-std in CI). (Size story.)
4. **v1.4+**: daemon mode for panels; then S2/S5/S7 guided by cargo-bloat data.
