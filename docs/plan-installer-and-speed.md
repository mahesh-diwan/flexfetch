# Plan: Robust, interactive installer + faster startup

Status: **Proposed** (not yet implemented)
Scope agreed with user: honest benchmarks + startup shave for the tool; **flags + smart defaults** for the installer (no interactive wizard); deliverable is this plan doc (no GitHub issue).

---

## Problem Statement

Two complaints, both measured against the current state:

**1. `install.sh` is not scriptable and not defensive enough.** It is 336 lines of good visuals (colors, spinner, progress bar, checksum verify, retry, PATH hint) but takes **no arguments at all** — no `--dry-run`, no `--version <tag>` pinning, no `--dir`, no `--no-confirm`. That makes it unusable in CI and package managers, and it fails late: dependencies (`curl`, `wget`, `tar`, `sha256sum`) are probed *mid-flight* rather than up front, and a Ctrl-C mid-download leaves a spinner process and a temp dir behind (no `trap`).

**2. `--benchmark` reports a number that doesn't match reality, and startup has a few shavable milliseconds.** Real measurements (release build, this machine):

| Scenario | Time |
|---|---|
| Warm full run | **5.4 ms** (median 6.45) |
| Cold cache (no `flexfetch-cache.json`) | **11 ms** |
| First run after boot (7.2 MB page-in) | **77 ms** |
| `--help` (clap parse floor) | 2.95 ms |
| `--version` | 2.81 ms |
| `--list-modules` | 3.16 ms |
| `--benchmark` reported "cold start" | **88 ms** ← misleading |

The 88 ms is an artifact: `bench::run` times each module **sequentially** to produce per-module numbers, so the headline "cold start" is the *sum* of sequential module times, not the real path. The real parallel collect (`run_selected`) is **2.7 ms**. Users see a fast tool but a lying benchmark, and the benchmark can't track regressions honestly.

Startup breakdown after clap: config load + cache file read + registry resolve + parallel collect + tera render ≈ 2.5 ms. The shavable pieces: the cache JSON is read **eagerly on every run** even when no cached module (`wifi`, `display`, `packages`, `bluetooth`, `media`, `publicip`) is selected, and the first-run 77 ms is dominated by the 7.2 MB binary page-in.

---

## Solution

**For the tool:** make `--benchmark` honest (report the real parallel end-to-end path as the headline, keep per-module cold timings as a labeled secondary table), then shave startup by making cache construction lazy (skip the JSON file read unless a cached module is selected) and trimming one redundant collect in `bench::run`. Target: warm run **5.4 → ~3 ms**, and a benchmark whose headline equals the stopwatch.

**For the installer:** add a small, documented flag surface with smart defaults — zero prompts by default, so the script keeps working everywhere it works today:

```
install.sh [--dry-run] [--version <tag>] [--dir <path>] [--no-confirm] [--check] [--quiet] [--help]
```

Plus defensive fixes: up-front dependency pre-check, `trap`-based cleanup (spinner + tmpdir on INT/TERM/EXIT), `--check` as an exit-code-friendly update check, `--dry-run` that prints the exact plan (tag, URL, target dir, size) and writes nothing.

---

## Commits

Each commit leaves the tree in a working state. Ordered so the benchmark fix lands first (it makes the later speed work measurable).

### Phase 1 — Honest benchmarks

**C1. `bench.rs`: split "cold sequential per-module" from "warm parallel" reporting.**
Rename the headline line from `cold start:` to something true — e.g. `cold sequential (per-module):` — and add a `real path (parallel collect + render):` line that measures what a normal invocation does: one `run_selected` + render with an already-parsed config. Keep the per-module table (it's genuinely useful for finding slow modules) but clearly label it "sequential, cold cache, informational".

**C2. `main.rs`: pass a real-path measurement into `bench::run`.**
`bench::run` already receives `t_cold_start`; add the parallel `run_selected` + render timing from the existing single-iteration branch so the benchmark's headline is the number a stopwatch would show for `flexfetch`.

**C3. `flag-smoke.sh`: assert the benchmark headline.**
Add a smoke check that `--benchmark` exits 0, prints both the per-module table and the real-path line, and that the real-path value is reported (not just the sequential sum).

### Phase 2 — Startup shave

**C4. `cache.rs`: make the file read lazy.**
`Cache::new` reads and parses `flexfetch-cache.json` eagerly. Change it so construction is cheap (empty map) and the file is read once on first `get`/`set` (or on first `get` only — writes still need the prior state to avoid clobbering; simplest correct approach: load on first `get`, and have `set` load-if-not-loaded first). `ttl` and `set_ttl` behavior unchanged. Unit tests in `cache.rs` (currently 0 tests — add): lazy load skips read until first access; expired entries still return `None`; `set` persists and reloads.

**C5. `context.rs`: only touch the cache file when a cached module is selected.**
`Context::with_fs`/`Context::new` constructs `Cache::new` unconditionally. Add a `cache: Option<Cache>` (or a `needs_cache: bool`) and have the module helpers that read the cache (`wifi`, `display`, `packages`, `bluetooth`, `media`, `publicip`) go through a `ctx.cache_mut()` that initializes on demand. `config_load.rs` sets the flag from the resolved module list. Net effect: runs whose preset contains none of the six cached modules (e.g. `--minimal`, `server`) skip the JSON read entirely. Verify the win with `--benchmark` before/after.

**C6. Re-measure and record.**
Run `--benchmark`, `hyperfine`-style repeated stopwatch runs (warm + cold cache), and update the numbers in this doc / CHANGELOG. Target: warm ≈ 3 ms, benchmark headline ≈ stopwatch.

Measured after C1–C5 (Aug 16): warm median **7.3 ms**, cold (no cache file) median **9.2 ms**, `--minimal` **7.3 ms** — first-run-after-build page-in still ~70 ms. The lazy cache's win is modest on the default preset (which does select wifi/display/packages) but real for minimal/server runs; the honest benchmark headline (`real path: collect … + render …`) is the substantive fix — it now equals the stopwatch instead of the sequential sum. Note: the clap parse floor (~2.9 ms) dominates the remaining budget; the earlier plan target of ~3 ms warm applies to `--minimal`-style runs, not the full default preset.

### Phase 3 — Installer: flags + smart defaults

**C7. `install.sh`: argument parser.**
Handle `--help`, `--dry-run`, `--version <tag>`, `--dir <path>`, `--no-confirm`, `--quiet`, `--check`, and `--` end-of-options. Unknown flag → print usage + exit 2. `--help`/`--version` (script version) exit 0. Keep current behavior identical when no flags are passed (backward compatible).

**C8. `install.sh`: up-front dependency pre-check.**
After OS/arch detection, verify `tar` and at least one of `curl`/`wget` plus one of `sha256sum`/`shasum` exist; fail with a clear message listing what's missing *before* any network work. `download()` and `verify_checksum()` keep their mid-flight checks as belt-and-braces.

**C9. `install.sh`: cleanup trap.**
`trap cleanup INT TERM EXIT` — kill the spinner (`spin_stop`), remove `$TMPDIR`. Today a Ctrl-C during download leaks both. Guard the trap body so it's a no-op before `TMPDIR` exists.

**C10. `install.sh`: `--dry-run`.**
Resolve the tag (same 3-tier logic), print the full plan: tag, exact download URL, checksum URL, target binary size (from the remote `Content-Length` if cheaply available, else "unknown"), and the install target directory that would be used. Write nothing, exit 0. Must not require the binary to already be installed.

**C11. `install.sh`: `--dir <path>`.**
Overrides the `INSTALL_DIR`/`~/.local/bin` fallback chain: install to exactly `<path>`, create it if needed, fail if not writable (no silent fallback when the flag is given). When `--dir` is absent, current behavior is preserved.

**C12. `install.sh`: `--check` + `--no-confirm` + `--quiet`.**
- `--check`: compare installed version vs latest tag; exit 0 = current, 1 = outdated, 2 = not installed, 3 = network/unknown. Prints one line. No writes. (CI-friendly.)
- `--no-confirm`: today the script never prompts, so this is a forward-compat flag that also suppresses the first-run demo/`tput`-style output; harmless and explicit for CI.
- `--quiet`: only errors and the final "installed" line.

**C13. `install.sh`: usage text + README.**
Document all flags in `--help` output and in the README install section. Note the existing curl one-liner stays as-is (no flags = same behavior).

### Phase 4 — Tests

**C14. CI: installer smoke.**
Add to `ci.yml` (or `flag-smoke.sh`): `bash -n install.sh`, `install.sh --help`, `install.sh --dry-run --version v0.0.0` (must exit 0 or 3 — no network dependency, writes nothing), and `install.sh --check` against a bogus tag returning a defined exit code. Run `shellcheck` if available in the runner image.

**C15. Regression: cache laziness + benchmark honesty tests.**
Rust unit tests from C4/C5 (cache lazy-load; context with no cached modules never reads the cache file — use `MockFs` with a `read` counter), plus the C3 smoke assertion.

---

## Decision Document

- **Benchmark semantics:** `--benchmark` keeps its per-module cold timings table (it's the tool for finding slow modules) but the headline becomes the real parallel path. The old "cold start: 88ms" line is removed, not renamed in place, to avoid two numbers that both look like "the time".
- **Cache laziness:** lazy *load-on-first-access*, not "skip cache entirely". Cache writes must still work for every module that uses them. A `set` when the file was never read must load first so it doesn't overwrite other modules' entries.
- **No interactive wizard:** agreed. The installer gets flags + smart defaults, not `read` prompts. This keeps it safe for `curl | sh` pipelines and package managers.
- **No dependency changes for the tool:** the speed work is confined to `bench.rs`, `cache.rs`, `context.rs`, `config_load.rs`, and `main.rs` plumbing. No new crates. The 7.2 MB binary diet (dropping zstd/image from default features) is **out of scope** — it would speed first-run page-in (77 ms) but changes the feature set; tracked separately.
- **wifi `iw` timeout:** the wifi module shells out to `iw` with no timeout; a hung `iw` stalls a cold run. Fixing this is a one-liner but touches module behavior — parked as a follow-up, not in this plan's commit list (out of scope for the startup-shave goal since wifi reads are cache-backed and warm runs are 5 ms).
- **Config `--flash`:** already skips config file IO — no change needed.
- **Logo loading:** already lazy (only the matched logo is parsed) — no change needed. 527 logos are not a startup cost.

## Testing Decisions

- **Good test = external behavior.** For the cache: "a run whose selected modules don't use the cache never reads the cache file" (assert via `MockFs` read count), and "a cached module reads the file exactly once on first access". Not "the internal Option field is set".
- **Prior art:** `flexfetch-core/src/modules/battery.rs` uses `MockFs` with `.file(...)` fixtures; `flexfetch-core/src/logo.rs` has the poisoning-recovery test pattern; `scripts/flag-smoke.sh` is the existing end-to-end flag harness. Cache tests follow the battery-module MockFs style; benchmark/installer checks extend `flag-smoke.sh`.
- **Modules tested:** `cache.rs` (lazy load, TTL, persistence), `context.rs` (no-cached-modules path), `bench.rs` output shape via smoke script, `install.sh` via `bash -n` + `--help` + `--dry-run` + `--check` in CI.
- **Performance regression guard:** the C6 recorded numbers (warm ≈ 3 ms, benchmark headline ≈ stopwatch) go in a comment near `bench::run`; `flag-smoke.sh` asserts the headline line exists so the reporting contract can't silently regress.

## Out of Scope

- Interactive wizard / prompts in `install.sh` (chosen: flags + smart defaults).
- Binary diet (7.2 MB → smaller; zstd/image feature reshuffle) — separate plan; would fix the 77 ms first-run page-in.
- `--sig` cosign verification in the installer (release pipeline already emits checksums; adding key verification is a follow-up).
- Shell-completions auto-install from the installer (exists as `flexfetch completions <shell>`; wiring it into install.sh is a possible follow-up).
- wifi `iw` timeout, live-mode sampler speed, `--watch` interval tuning.
- Windows support for the installer (Linux/macOS only, as today).

## Further Notes

- The 65 ms `battery` figure in the per-module table is a cold-sequential artifact (sysfs reads under a cold page cache in the timing loop); in the real parallel path it's hidden. Worth confirming with the C6 re-measure that no single module dominates the real path.
- All measurements above were taken on this machine (release build, `--no-default-features` not measured; the diet work is separate). Re-measure on CI (the existing `bench.yml` workflow) after C6.
- The plan intentionally lands the honest-benchmark commit *first*: every later speed commit is then verifiable by "benchmark headline went down".
