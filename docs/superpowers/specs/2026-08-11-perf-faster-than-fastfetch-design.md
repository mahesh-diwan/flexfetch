# Design: Make flexfetch snappier than fastfetch

Date: 2026-08-11
Status: Approved

## Goal

Default-config `flexfetch` wall time (pipe _and_ interactive TTY) must land
**below fastfetch's** default (~22 ms measured on this machine). Current
state: ~60 ms typical, ~480 ms on a cold public-IP cache.

## Measured baseline (CachyOS, pipe mode, release builds)

| Case                                            | median  | min     |
| ----------------------------------------------- | ------- | ------- |
| `fastfetch --pipe`                              | 22.0 ms | 18.4 ms |
| `flexfetch --pipe` (full, default preset)       | 63 ms   | 49 ms   |
| `flexfetch --minimal --pipe` (full build)       | 24.4 ms | 17.3 ms |
| `flexfetch --minimal --export csv` (full build) | 7.5 ms  | 5.7 ms  |
| `flexfetch --minimal --pipe` (minimal build)    | 3.5 ms  | 2.7 ms  |

Cost structure (from `--benchmark` + per-module isolation):

- `publicip` — 300 ms: WAN TCP round-trip to api.ipify.org on cache miss (60 s
  TTL cache). In the default preset.
- `wifi` — 50 ms: `nmcli` subprocess + NetworkManager D-Bus round-trip.
- `cpuusage` — 30 ms: fixed 30 ms sleep sampling a `/proc/stat` delta.
- **~17 ms floor: `Tera::new` recompiles the built-in `default` template on
  every process spawn** — the delta between the csv path (7.5 ms) and the
  pipe render path (24 ms) for identical tiny module sets. Confirmed:
  `CACHED_TERA` is a per-process `OnceLock`.
- Remaining ~4 ms floor (full vs minimal build with template bypassed):
  bigger binary page faults + rayon pool init + setup. Accepted for now.

## Design

### 1. Native fast path for the built-in default template (~17 ms → ~1 ms)

When the active template is the built-in `default` (i.e. no custom template
file configured), `TeraEngine::render` routes to the **existing native
`render_plain`** path instead of compiling a Tera instance. `default.tera`
remains the source of truth for the default layout; `render_plain` already
mirrors it (it is the no-`tera`-feature fallback today).

- Change surface: a branch in `TeraEngine::render` keyed on
  `config.template == "default"` (or equivalent "built-in default, no
  overrides" signal); the `tera` feature still ships Tera for real custom
  templates.
- Data flow: `run_selected` collects `SystemInfo` → native render produces
  title/separator/rows/sections → logo drawn by the existing post-`render`
  block in `render()` → unchanged API.
- Behavioral guard: a **golden test** renders one fixed `SystemInfo` through
  both `render_tera` and `render_plain` and asserts identical output, so the
  two paths cannot silently drift.
- Fallback equality: any divergence found by the golden test is fixed by
  aligning `render_plain`, never by re-enabling Tera for the default path.

### 2. Wifi native fast path (50 ms → ~2 ms)

Replace the `nmcli` subprocess with, in order:

1. `/proc/net/wireless` — active interface + signal quality (always-stable).
2. `/sys/class/net/<if>/operstate` + `flags` — confirm link is up.
3. SSID via `iwgetid -r <if>` (tiny C util, ~1–2 ms spawn). If `iwgetid` is
   missing or fails, fall back to the existing `nmcli` path; if both fail,
   emit `unknown` / `not connected` as today.

No new dependency. macOS branch unchanged (existing `WifiModule` path or
`unknown`).

### 3. cpuusage: single read, since-boot average (30 ms → ~0.1 ms)

Drop the `thread::sleep(30ms)` delta loop. One `/proc/stat` read computes
`(total − idle) / total` over the cpu line — usage averaged since boot. Free,
always stable. Value semantics change from "recent snapshot" to "since-boot
average"; label unchanged.

### 4. publicip: out of the default preset (0–300 ms → not in default runs)

Remove `publicip` from `Config::default_modules()`. Still reachable via
`--modules publicip` and the `full` preset. The existing 60 s TTL disk cache
stays as-is for opt-in users.

## Out of scope (tracked, not implemented)

- `weather` module (646 ms opt-in WAN fetch) — add a persistent TTL cache in a
  later pass if wanted.
- `display` module (4–13 ms) — re-verify after the above fixes.
- Rayon pool init (~2 ms) — re-verify after the above fixes.

## Success criteria

1. Default preset, pipe, warm runs: flexfetch median < fastfetch median
   (~22 ms), measured with the same python spawn loop used for the baseline.
2. Interactive TTY default run feels instant (no perceptible delay on each
   shell invocation); logo still renders.
3. `--modules publicip` still works; `full` preset still includes it.
4. Custom-template users (real `.tera` file) get identical output, via Tera
   as today.
5. `cargo test --workspace` green; golden render test passes; criterion
   `cold_start` gate not regressed.
