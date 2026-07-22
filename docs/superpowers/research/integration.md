# Competitive Analysis: System Info Tools

## Feature Comparison Table

| Feature                    | flexfetch                                           | fastfetch                                         | neofetch               | macchina                            | pfetch          |
| -------------------------- | --------------------------------------------------- | ------------------------------------------------- | ---------------------- | ----------------------------------- | --------------- |
| **Language**               | Rust                                                | C                                                 | Bash                   | Rust                                | POSIX sh        |
| **Speed**                  | Fast                                                | ~15ms                                             | ~364ms                 | ~12ms                               | ~45ms           |
| **Binary size**            | ~5 MB                                               | ~2 MB                                             | ~1 KB (script)         | ~1.8 MB                             | ~15 KB (script) |
| **Lua plugins**            | Yes                                                 | No                                                | No                     | No                                  | No              |
| **Tera templates**         | Yes                                                 | No                                                | No                     | No                                  | No              |
| **Theme presets**          | 5 (Catppuccin, Dracula, Nord, Gruvbox, Tokyo Night) | JSONC presets (built-in + community)              | env vars (3 colors)    | External theme files                | 3 env vars      |
| **Config format**          | TOML                                                | JSONC                                             | Bash script            | TOML (theme separate)               | env vars        |
| **Output formats**         | text, JSON                                          | text, JSON, JSONC                                 | text                   | text, JSON                          | text            |
| **Pipe detection**         | No (manual flag)                                    | Yes (`--pipe`, auto via `isatty`)                 | No                     | No                                  | No              |
| **Image rendering**        | No                                                  | Sixel, Kitty, Kitty-direct, iTerm2, Chafa         | No                     | No                                  | No              |
| **Module toggle groups**   | Manual (`-m`)                                       | `-c all`, `--structure`, presets                  | `--flag` per module    | `--doctor` for debug                | None            |
| **Logo count**             | 7 + generic                                         | ~200                                              | ~150                   | ~20                                 | small           |
| **Parallel detection**     | Yes (Rayon)                                         | Yes                                               | No                     | Yes                                 | No              |
| **Custom modules**         | `[custom]` config + Lua plugins                     | Command module (shell scripts)                    | Custom lines in config | libmacchina extensions              | None            |
| **JSON schema validation** | No                                                  | Yes (editor intelligence)                         | No                     | No                                  | No              |
| **OS/platform support**    | Linux, macOS                                        | Linux, macOS, Windows, Android, BSD, Haiku, SunOS | ~150 OSes              | Linux, macOS, Windows, BSD, Android | Linux, macOS    |

## Fastfetch Success Factors (23.8K stars)

1. **Speed + polish** — Sub-millisecond on fast hardware. More precise output (555.00 MiB vs neofetch's 555 MiB). Active development (230 contributors, 180 releases).
2. **JSONC configuration** — IDE schema validation, comments in config, structured module objects with per-module formatting. Users get editor autocomplete.
3. **Preset system** — Built-in presets in `presets/` directory. `--config neofetch` for migration. `--gen-config` / `--gen-config-full` for quick start.
4. **Image rendering** — 6 terminal protocols (Sixel, Kitty, Kitty-direct, iTerm2, Chafa). Image caching. This is a major differentiator for screenshot culture.
5. **Pipe detection** — Auto-disables colors via `isatty(1)`. `--pipe` flag for explicit control. Essential for scripting and piping to files.
6. **Accuracy** — Wayland support, proper memory stats, detailed platform-specific detection. Neofetch's inaccurate output drove users away.
7. **Module structure** — `--structure` flag, `--structure-disabled`, colon-separated module lists. Full control over what shows and in what order.

## Neofetch Decline Factors

1. **Abandoned** — Last update 2021, archived 2024. Creator "took up farming." No Wayland support, no Windows support, no new distros.
2. **Slow** — 364ms average execution. Bash interpreter overhead on every invocation.
3. **Inaccurate** — Wrong memory stats, missing Wayland, broken on many modern setups.
4. **No JSON output** — Can't integrate with scripts or tooling.
5. **No config schema** — 10,000+ line bash config file, no validation, no editor support.
6. **Cultural momentum** — Screenshots of `neofetch` output became Linux memes/social proof. Fastfetch captured this audience by being a drop-in replacement.

## Macchina Strengths

- **Performance champion** — 12ms, fastest tested. Rust-native.
- **Theme system** — Themes live outside config files, shareable as standalone files.
- **`--doctor` flag** — Built-in troubleshooting for failed fetches. Good UX.
- **JSON output** — Machine-readable. Security audit use case.
- **Maintenance mode** — Stable, not actively adding features. Low risk.

## Pfetch Strengths

- **Minimalism** — Single POSIX shell script, zero runtime deps. Perfect for containers, embedded.
- **Speed** — 45ms, faster than neofetch by 8x.
- **Simplicity** — env vars for config. No config files to manage.
- **Small footprint** — 15KB script.

## Top 5 Features to Adopt (Ranked by Impact vs Effort)

### 1. Pipe Detection (HIGH impact, LOW effort)

**What:** Auto-detect `isatty(1)` on stdout. Disable colors, skip logo when not a TTY. `--pipe` flag for explicit control.
**Why:** Every other tool has this. Essential for scripting. Users already complain about piping flexfetch output.
**Effort:** ~10 lines. Check `std::io::stdout().is_terminal()` in Rust.
**Skip when:** Never. This is table stakes.

### 2. Module Toggle Groups (HIGH impact, LOW effort)

**What:** Named module bundles: `--minimal` (title, os, kernel, uptime), `--full` (everything), `--dev` (title, os, cpu, memory, disk, shell, term). Maps to `-m` flag internally.
**Why:** Users don't want to remember module names. Named groups reduce friction.
**Effort:** ~20 lines. HashMap of preset names to module lists.
**Skip when:** Never.

### 3. Preset System (MEDIUM impact, MEDIUM effort)

**What:** Named bundles of theme + modules + logo. Stored in `~/.config/flexfetch/presets/`. `flexfetch --preset minimal`. `flexfetch --list-presets`. Include community presets in repo.
**Why:** fastfetch's preset system is their most-loved feature. Shareable, discoverable, composable.
**Effort:** ~100 lines. TOML deserialization of preset files, merge logic with config.
**Skip when:** If Lua plugins + themes already cover this use case (they partially do).

### 4. Color Scheme Sharing (MEDIUM impact, LOW effort)

**What:** Export/import named color schemes. `flexfetch --export-theme dracula > my-theme.toml`. `flexfetch --import-theme my-theme.toml`. Share themes via single TOML file.
**Why:** Community theme sharing is social proof and drives adoption. fastfetch's JSONC presets are shared constantly on Reddit/HN.
**Effort:** ~30 lines. Serialize theme struct to TOML, deserializable.
**Skip when:** If preset system (#3) includes theme sharing inherently.

### 5. JSON Schema Validation (LOW impact, LOW effort)

**What:** Ship a JSON schema for config.toml. Add `$schema` comment in generated config. Enable editor autocomplete/validation.
**Why:** fastfetch users love this. Reduces config errors, improves DX.
**Effort:** ~50 lines to generate schema, or hand-maintain. Zero runtime cost.
**Skip when:** If TOML ecosystem has native schema support (check `taplo`).

## Top 5 Anti-Patterns to Avoid

### 1. Feature Bloat — Too Many Options

**What:** fastfetch has 200+ config options. Many users only use 5. The `--gen-config-full` output is overwhelming.
**Avoid by:** Keep module count at 20. Let Lua plugins handle niche needs. Each option must pass the "would >30% of users need this?" test.

### 2. Slow Startup from Script Interpretation

**What:** neofetch's 364ms comes from Bash interpreting a 10,000-line script on every invocation.
**Avoid by:** Rust binary. Static compilation. No runtime interpreters (Lua is optional, not required).

### 3. Breaking Theme Format Changes

**What:** fastfetch changed theme config format between v1.x and v2.x, breaking existing configs silently.
**Avoid by:** Semver. Config migration tool (`flexfetch --migrate-config`). Version field in config file.

### 4. No Pipe Detection / Manual Color Control

**What:** flexfetch currently has no auto-detection. Users must manually pass `--no-color`. Other tools solve this automatically.
**Avoid by:** Ship pipe detection (#1 in features). It's 10 lines and eliminates a class of support issues.

### 5. Complex Config Formats

**What:** fastfetch's JSONC is powerful but verbose. neofetch's bash config is unreadable. Both lose users who just want quick output.
**Avoid by:** Keep TOML config flat and readable. sensible defaults. `--gen-config` should produce a 10-line file, not 100 lines.

## Recommended Feature Roadroad

### Phase 1: Table Stakes (v0.4)

- [ ] Pipe detection (`isatty` + `--pipe` flag)
- [ ] Module toggle groups (`--minimal`, `--full`, `--dev`)
- [ ] Add more ASCII logos (target: 20+)

### Phase 2: Community (v0.5)

- [ ] Preset system (`--preset`, `--list-presets`, `--save-preset`)
- [ ] Theme export/import
- [ ] JSON schema for config.toml
- [ ] `--gen-config` produces minimal config

### Phase 3: Power Users (v0.6)

- [ ] Image logo support (Sixel + Kitty protocols)
- [ ] `--stat` flag (per-module timing)
- [ ] `--doctor` flag (diagnose failed modules)
- [ ] Config migration tool (`--migrate`)

### What to Skip

- **Windows support** — Not enough demand for v1. Focus on Linux/macOS.
- **200+ ASCII logos** — Diminishing returns. 20-30 covers 95% of users.
- **JSONC config** — TOML is already good. Don't add config format complexity.
- **Kitty-direct protocol** — Edge case. Ship Sixel first (widest support).
