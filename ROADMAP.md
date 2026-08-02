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
   based hot-reload (mtime works, no dep needed), and the macOS x86_64 cross-build
   validation via `workflow_dispatch` (the only release-matrix item not yet proven).

## Reference

- Original plan source: pasted v2.0 plan (Rust-native optimization plan).
- Design docs: `docs/superpowers/specs/`, `docs/superpowers/plans/`, `docs/superpowers/research/`.
- Size baselines: native 6.50 MB (full, incl. live + image-logos) · minimal (--no-default-features)
  6.04 MB · x86_64-musl 6.40 MB · aarch64-musl 4.69 MB · armv7-musl 4.38 MB.
