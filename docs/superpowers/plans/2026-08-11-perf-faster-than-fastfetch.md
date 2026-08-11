# Perf: faster than fastfetch

**who** flexfetch maintainer · **when** 2026-08-11 · **why** default-config wall time (pipe + interactive TTY) must beat fastfetch's ~22 ms
**main idea** Kill the ~20 ms Tera compile+render with a native fast path — every other cost is already ~6.5 ms.

Baseline (full release build, median over 25+ spawns):

| path                           | before  |
| ------------------------------ | ------- |
| fastfetch --pipe               | 22.0 ms |
| flexfetch default --pipe       | 63 ms   |
| flexfetch default --export csv | 7.5 ms  |
| flexfetch --minimal --pipe     | 27.2 ms |

Probe findings that shape this plan:

- Stubbing `render_logo` → no change (28.7 vs 27.2 ms). Logo render is ~free. The ~20 ms is **entirely** Tera compile + render + context build.
- `render_plain` already emits Nerd-Font icons and is **byte-identical** to `render_tera` output (verified on minimal set and a 23-module set, TERM=kitty; only diff was a live memory %). No icon porting needed.
- `render_plain` (:952), `dedup_visible` (:1066), `section_for` (:258) are `#[cfg(not(feature = "tera"))]` — must be un-gated to run in the release (tera) build.

Target: default `--pipe` ≤ **15 ms** (≪ fastfetch), interactive TTY subjectively instant, zero render-output change.

---

## Phase A — native fast path for the built-in default template

File: `flexfetch-core/src/template.rs`

1. Un-gate for all builds: remove `#[cfg(not(feature = "tera"))]` from `render_plain` (:952), `dedup_visible` (:1066), `section_for` (:258). (`label_for`, `plain_value` are already ungated.)
   - Use `#[cfg_attr(not(feature = "tera"), allow(dead_code))]` if tera-only callers pull an unused warning; prefer keeping both call sites live so nothing is dead.
2. In `TeraEngine::render` (:492), replace the feature branch with a template-value branch:

   ```rust
   #[cfg(feature = "tera")]
   let raw = if config.template == "default" {
       render_plain(info, config)
   } else {
       self.render_tera(info, config)?
   };
   #[cfg(not(feature = "tera"))]
   let raw = render_plain(info, config);
   ```

   No custom template path exists today, so `render_tera` stays reachable as the seam only.

3. Golden test (within template.rs test module, `#[cfg(feature = "tera")]`): build a small **fixed** `SystemInfo` (no live modules — memory %, cpu% drift) and assert `render_plain(&info, &config) == render_tera(&info, &config)?` byte-for-byte. Terrain covered: title/separator rows, dedup (de/wm), a value with a bar (memory), a `Scalar`, a `Map`.
4. Verify: `cargo build --release` full + `cargo build --release --no-default-features`; re-run the tera-vs-plain spawn diff (must stay identical); `cargo test --workspace`.

## Phase B — wifi: native fast path + iwgetid, nmcli fallback

File: `flexfetch-core/src/modules/wifi.rs`

1. Read active wifi iface + signal from `/proc/net/wireless` (skip header line `Inter-| face`; active = any interface line whose `quality` field > 0; take the best). Signal maps 0–70 → 0–100 %.
2. SSID via `iwgetid -r <iface>` (`Command`); on nonzero exit → not connected.
3. Checks happen through `ctx.read_file(...)` so tests never touch real `/proc`; `iwgetid`/`nmcli` spawns stay `Command` but are the fallback tier only.
4. Fallback chain: native `/proc/net/wireless` + `iwgetid` → existing `nmcli -t ... --rescan no` full parse (kept verbatim) → "not connected" / "unknown". Output `InfoValue::Map` with `ssid`, `signal`, `frequency`, `security` when known; omit unknown keys (renderer labels by key presence).
5. Unit test: fake `/proc/net/wireless` + sysfs root via ctx → asserts ssid/signal mapping; disconnected case → falls back.
6. Verify: `--benchmark` wifi drops from ~52 ms to < 2 ms; live `--pipe` still shows wifi row.

## Phase C — cpuusage: single read, since-boot average

File: `flexfetch-core/src/modules/cpuusage.rs`

1. Delete the `30 ms sleep` delta (`read_usage`, :21–48). Single `/proc/stat` aggregate line: `total = Σ(user..steal)`, `idle = idle + iowait`, avg = `(total−idle)/total × 100`.
2. Read through `ctx.read_file` for testability; unit test asserts known synthetic `/proc/stat` → expected %.
3. Verify: `--benchmark` cpuusage drops from ~30 ms to < 1 ms; value stable and plausible (since boot).

## Phase D — publicip out of the default preset

Files: `flexfetch-core/src/config.rs`, `flexfetch-core/src/presets.rs`

1. Remove `"publicip".into()` (:344) from `default_modules()`.
   - `presets.rs` `"default"` and `"full"` both equal `default_modules()` — the `full` preset loses it too. Fix: make `full` its own list, `default_modules()` + `"publicip"` (`"full" => { let mut v = Config::default_modules(); v.push("publicip".into()); v }`).
2. Module stays available via `--modules publicip`.
3. Verify: default `--pipe` shows no publicip row (no WAN TCP at all); `--preset full` still does.

## Phase E — verify gate + ship

1. `cargo test --workspace` (new wifi/cpuusage/golden tests included), `cargo clippy --all-features`, `rustfmt`.
2. Criterion `cold_start` bench (3 MiB size gate in `.github/workflows/bench.yml`) — expect large drop; keep gate.
3. Head-to-head python-spawn loop vs fastfetch: warm + cold, default pipe, `--minimal`, interactive TTY eyeball.
4. `--benchmark` per-module: publicip ~298 ms gone from default, wifi < 2 ms, cpuusage < 1 ms.
5. Bump version in Cargo.toml → commit → tag `v0.31.0` → `git push --tags` (CI rel + site per project preference).

---

## Risks / non-goals

- Icon parity: already proven byte-identical; golden test locks it. No image-logos/osc8/truecolor changes in scope.
- `render_tera` seam: dead today but kept; no custom-template path is being built (separate feature).
- wifi native: rare NICs without `/proc/net/wireless` fall through to nmcli — correctness preserved.
- Output change python in `cpuusage %` semantics (now since-boot) and wifi security/frequency keys may be absent — accepted and reflects real data.
- Out of scope (re-verify after landing): weather TTL cache, `display` module, rayon pool init.
