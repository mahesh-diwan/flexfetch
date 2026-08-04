# Fastfetch Benchmark — why it looks so good & runs so fast

> Research captured Aug 2026. Source: fastfetch's public docs + README, flexfetch
> codebase audit (`theme.rs`, `template.rs`, `logo.rs`, `image_logo.rs`,
> `default.tera`, `Cargo.toml` feature sets, `module_registry.rs`), and the
> earlier design brainstorm (`.superpowers/brainstorm/`). Every "gap" below was
> verified against the current tree; the ROADMAP "Phase 7" section is the
> canonical backlog tracker.

---

## Part 1 — The 8 pillars of fastfetch's looks

### 1. Truecolor themes (the single biggest visual gap)

**fastfetch:** themes are defined as truecolor hex (`#89b4fa`) and emitted as
`38;2;R;G;B` with a graceful ladder down to 256-color → 16-color → none.

**flexfetch today:** all 27 theme presets use **16-color ANSI** (`\x1b[94m`,
`\x1b[95m`…). This is why output looks "neon cheap" next to fastfetch's soft
pastel gradients. `gradient_text()` in `theme.rs` already emits truecolor —
but the static key/value/sep slots do not.

**Fix:** keep presets as `[u8;3]` RGB (we already store `gradient_colors`),
emit `38;2;R;G;B`, degrade on `COLORTERM`/`TERM` detection. Biggest aesthetic
win per line of code.

### 2. Perfect alignment (wide-glyph aware)

**fastfetch:** pads keys to the max module width and handles double-width
glyphs (CJK + Nerd Font icons).

**flexfetch:** `visible_len()` (`logo.rs`) counts every char as 1 cell. Many
Nerd Font icons are 2 cells — icon'd rows misalign by one column.

**Fix:** use `unicode-width` (or a small wcwidth) in `pad_filter` /
`visible_len`. Subtle, but it's what makes rows line up *perfectly*.

### 3. Per-module colors

**fastfetch:** `keyColor` / `outputColor` per module (CPU yellow, Memory green…).

**flexfetch:** global `color_keys`/`color_values` only. `themes.md` research
already designed the TOML shape (`[[display.modules]] type="cpu" key_color=...`).

### 4. Image logos

**fastfetch:** kitty/sixel/iTerm2 real PNG rendering.

**flexfetch:** `image_logo.rs` already has `ImageProtocol::detect()`
(Kitty/Iterm2/Sixel/Block) + distro logo path resolution — **already strong.**
Gaps: default-on behavior, and the kitty/sixel path only fires when logo files
exist on disk.

### 5. Brand-colored logos

**fastfetch:** auto-colors ASCII logos with the distro's brand palette
(Arch blue→cyan fade).

**flexfetch:** logos are single-color (`${1}` cyan). No per-line brand gradient.

**Fix:** extend the `${N}` placeholder system with a small gradient over the
logo's line indices.

### 6. Section grouping

**fastfetch:** groups modules into titled sections with subtle separators
(`── Hardware ──`).

**flexfetch:** `box_chars` exist (double/dotted/thick/rounded) but the default
template is flat rows.

**Fix:** a `{% if section %}` grouping pass in `default.tera`.

### 7. Data-rich visuals

**fastfetch:** mini bar graphs, battery glyphs with levels, temp colored
hot→cold.

**flexfetch:** has `progress_bar` + `palette_display` filters and `--live`
ratatui gauges, but the static output only uses them sparsely.

**Fix:** wire bars into `cpuusage`/`memory`/`disk` rows + color-by-threshold
(green<60, yellow<85, red≥85 — `progress_bar_filter` already does this).

### 8. Terminal-aware polish

**fastfetch:** OSC-8 hyperlinks (clickable host/IP), truecolor detection, Nerd
Font detection with fallback.

**flexfetch:** `terminal.rs` already detects OSC-8 + image protocols +
truecolor.

**Fix:** emit `\x1b]8;;https://…\x1b\\` on `publicip`/`host`, and auto-disable
icons when no Nerd Font.

---

## Part 2 — The 5 pillars of fastfetch's speed

### ⚠️ The finding that matters most

The **release build is `--no-default-features --features live,image-logos,completions`**
(`.github/workflows/release.yml`) — which does **NOT** include `parallel` or
`tera`! So the shipped binary:

- **collects modules sequentially** (no rayon — the `parallel` feature is dropped),
- uses the **plain renderer**, not Tera.

That was the single biggest speed lever sitting unused: the parallel collector
exists (`module_registry.rs` `par_iter`), it just wasn't in the release feature
set. **Resolved Aug 2026:** `release.yml` now builds with
`live,image-logos,completions,parallel` (task 7.9). Tradeoff weighed: rayon was
deliberately excluded by the 0.2 diet to keep the release binary ~2 MB —
re-adding `parallel` costs ~100–200 KB (still well under the 4 MB size gate),
a conscious speed↔size decision, now made.

### 1. Zero subprocess spawns

**fastfetch:** reads `/proc`, `/sys`, and libc syscalls directly.

**flexfetch:** Phase 4.1 already zero-spawned the hot modules
(kernel/packages/wm/disk/health/network/wifi/publicip/cpuusage). **But 33
`Command::new` calls remain** across the default path: `ps -e` (processes),
`free -h` (swap), `xrandr`/`wlr-randr` (display/resolution), `sensors` (temp),
`lspci` (gpu fallback), `gsettings` (wm/wallpaper), `nmcli`, `bluetoothctl`,
`git`, `dbus-send`… Each spawn costs 1–5 ms of fork/exec + parsing.

**Fix priority:** `processes` (`/proc` scan), `swap` (`/proc/meminfo`),
`resolution` (parse `xrandr` is slow — cache or EDID), `temperature`
(`/sys/class/thermal`).

### 2. Parallel module collection

**fastfetch:** pthreads, one thread per module.

**flexfetch:** rayon `par_iter` exists — see the release-feature finding above.
Adding `parallel` to the release profile is the cheapest fix on this list.

### 3. No runtime template engine

**fastfetch:** direct string building, zero parsing.

**flexfetch:** Tera (default build) is OnceLock-cached (good) but still
parses/renders; the release build drops it entirely and uses `render_plain`
(fast). So the *fast* path already exists — the tradeoff is the minimal
renderer has fewer features. Keep both, keep the cache.

### 4. Caching

**flexfetch:** `cache.rs` + publicip TTL cache + `OnceLock` Tera + logo
`OnceLock` cache — already done. `--watch`/`--live` reuse snapshots.

### 5. Measured gates

`--benchmark` + the `perf-gate` CI job (hyperfine, fail >10 ms on `--minimal`)
— already in place. Cold start went 5.5 s → 686 ms (debug) / sub-10 ms minimal
(release).

---

## Part 3 — Aesthetics backlog (ranked)

| Tier | Idea | Effort | Payoff | Status |
| ---- | ---- | ------ | ------ | ------ |
| 🥇 | **Truecolor theme slots** (hex RGB → `38;2` with 256/16 fallback) | Low | Huge — instantly "expensive" look | ✅ (Aug 2026) |
| 🥇 | **Unicode-width padding** (fix Nerd Font misalignment) | Low | High — rows line up perfectly | ✅ (Aug 2026) |
| 🥈 | **Per-module key colors** (`[[display.modules]]`) | Medium | High — fastfetch's signature look | ✅ (Aug 2026) |
| 🥈 | **Bars + thresholds** on cpu/mem/disk rows | Low | High — data becomes visual | ✅ (Aug 2026) |
| 🥈 | **Section headers** with subtle separators | Medium | Medium | ⬜ |
| 🥈 | **Logo brand gradients** (per-line color fade) | Medium | Medium | ✅ (Aug 2026) |
| 🥉 | **OSC-8 hyperlinks** (clickable host/public IP) | Low | Nice wow | ✅ (Aug 2026) |
| 🥉 | **Nerd Font auto-detect + ASCII icon fallback** | Medium | Nice | ✅ (Aug 2026) |
| 🥉 | **Battery glyph with level** (🔋 79%) | Low | Nice | 🟡 (bars cover it; glyph pending) |
| 🥉 | **`--list-themes` live preview** (swatch rows per theme) | Low | Great discoverability | ✅ (Aug 2026) |
| 🥉 | **Random/cycle theme** (`--theme random`) | Low | Fun | ✅ (Aug 2026) |

## Part 4 — Speed backlog (ranked)

| # | Idea | Effort | Payoff | Status |
| - | ---- | ------ | ------ | ------ |
| 1 | **Add `parallel` (rayon) to the release feature set** | Trivial | High — shipped binary collects concurrently | ✅ (Aug 2026) |
| 2 | **Zero-spawn remaining hot collectors** (`processes`, `swap`, `temperature`, `resolution`) | Medium | High | ✅ (Aug 2026, Linux) — processes reads `/proc`, swap reads `/proc/meminfo`, temperature reads `/sys/class/thermal`, resolution reads DRM `modes` sysfs; remaining `Command::new` are macOS-only or fallbacks |
| 3 | **`--smart`/`--watch` snapshot reuse** (skip re-collect on no-change) | Medium | Medium | ⬜ |

---

## References

- fastfetch: https://github.com/fastfetch-cli/fastfetch (README + docs)
- flexfetch theme system: `flexfetch-core/src/theme.rs`
- flexfetch renderers: `flexfetch-core/src/template.rs`, `templates/default.tera`
- flexfetch logos: `flexfetch-core/src/logo.rs`, `image_logo.rs`, `fastfetch_logos.rs`
- flexfetch feature matrix: `flexfetch-core/Cargo.toml`, `flexfetch-cli/Cargo.toml`
- Prior brainstorm: `.superpowers/brainstorm/657024-1784801570/content/`
- Related research: `docs/superpowers/research/themes.md`, `performance.md`, `customization.md`
