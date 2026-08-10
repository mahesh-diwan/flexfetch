# Architecture Deepening Phase 2 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Eliminate cross-crate duplication, move static data to core, extract logo data from logic, and add test coverage for the two largest untested files.

**Architecture:** Extract `info_value_summary` to `InfoValue` as a method (core), move `builtin_presets`/`module_group` to core, extract logo const data to `logo_data.rs`, add tests for `theme::resolve` and `logo::detect`/`visible_len`.

**Tech Stack:** Rust 1.75+, existing crate structure (flexfetch-core, flexfetch-cli)

## Global Constraints

- All existing tests must pass after each task
- No breaking changes to public API (flexfetch-core crate)
- Feature flags preserved
- Platform branching preserved (#[cfg(target_os = "...")])
- Minimum Rust version: 1.75

---

## File Structure

| File                              | Responsibility                                                                           |
| --------------------------------- | ---------------------------------------------------------------------------------------- |
| `flexfetch-core/src/export.rs`    | Remove adapter shims (pub mod image/text/structured), keep export functions at top level |
| `flexfetch-core/src/module.rs`    | Add `info_value_summary()` method to `InfoValue`                                         |
| `flexfetch-core/src/presets.rs`   | **NEW** — `builtin_presets()`, `module_group()`, `load_preset()`                         |
| `flexfetch-core/src/logo.rs`      | Keep logic only (~220 lines), import data from logo_data                                 |
| `flexfetch-core/src/logo_data.rs` | **NEW** — all const Logo data (717 lines)                                                |
| `flexfetch-core/src/theme.rs`     | Add unit tests for `resolve`, `find_preset`, `resolve_ansi`                              |
| `flexfetch-cli/src/main.rs`       | Import presets from core, remove duplicate `info_value_summary`                          |

---

## Task 1: Extract `info_value_summary` to `InfoValue` method

**Files:**

- Modify: `flexfetch-core/src/module.rs` (add method to `InfoValue`)
- Modify: `flexfetch-core/src/export.rs:443-454` (remove private fn, use method)
- Modify: `flexfetch-cli/src/main.rs:573-584` (remove duplicate, use method)

**Interfaces:**

- Consumes: `InfoValue` enum (existing)
- Produces: `InfoValue::summary(&self) -> String`

- [ ] **Step 1: Add `summary()` method to `InfoValue` in `flexfetch-core/src/module.rs`**

```rust
// After the InfoValue enum definition (around line 362), add:

impl InfoValue {
    /// Compact one-line summary for diff tables and exports.
    pub fn summary(&self) -> String {
        match self {
            InfoValue::Scalar(s) => s.clone(),
            InfoValue::Map(m) => {
                let mut parts: Vec<String> = m.iter().map(|(k, val)| format!("{k}={val}")).collect();
                parts.sort();
                parts.join(", ")
            }
            InfoValue::List(l) => l.join(", "),
            InfoValue::Table(t) => format!("{} rows", t.len()),
        }
    }
}
```

- [ ] **Step 2: Run existing tests to verify no breakage**

Run: `cargo test -p flexfetch-core`
Expected: PASS

- [ ] **Step 3: Update `export.rs` to use `InfoValue::summary()`**

In `flexfetch-core/src/export.rs`:

- Delete the private `fn info_value_summary(v: &InfoValue) -> String` (lines 443-454)
- Replace all calls to `info_value_summary(value)` with `value.summary()`

- [ ] **Step 4: Update `main.rs` to use `InfoValue::summary()`**

In `flexfetch-cli/src/main.rs`:

- Delete the private `fn info_value_summary(v: &InfoValue) -> String` (lines 573-584)
- Replace calls to `info_value_summary(v)` with `v.summary()` in `render_diff`

- [ ] **Step 5: Run all tests**

Run: `cargo test --workspace`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add flexfetch-core/src/module.rs flexfetch-core/src/export.rs flexfetch-cli/src/main.rs
git commit -m "refactor: extract info_value_summary to InfoValue::summary() method"
```

---

## Task 2: Remove dead export adapter shims

**Files:**

- Modify: `flexfetch-core/src/export.rs:563-605` (delete pub mod image/text/structured)

**Interfaces:**

- Consumes: existing export functions (unchanged)
- Produces: simpler export.rs without pass-through modules

- [ ] **Step 1: Delete the adapter shims**

In `flexfetch-core/src/export.rs`, delete lines 563-605:

```rust
// DELETE these three modules entirely:
pub mod image { ... }
pub mod text { ... }
pub mod structured { ... }
```

- [ ] **Step 2: Check if any code imports from these shims**

Run: `rg "export::(image|text|structured)" --type rust`
Expected: No results (or only in test code that can be updated)

- [ ] **Step 3: Update any imports if found**

If any code uses `flexfetch_core::export::image::export_svg`, change to `flexfetch_core::export::export_svg`.

- [ ] **Step 4: Run tests**

Run: `cargo test --workspace`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add flexfetch-core/src/export.rs
git commit -m "refactor: remove dead export adapter shims (image/text/structured modules)"
```

---

## Task 3: Move preset data to core

**Files:**

- Create: `flexfetch-core/src/presets.rs`
- Modify: `flexfetch-core/src/lib.rs` (add `pub mod presets`)
- Modify: `flexfetch-cli/src/main.rs` (import from core, remove local functions)

**Interfaces:**

- Consumes: `Config::default_modules()` (existing)
- Produces: `presets::builtin_presets()`, `presets::module_group()`, `presets::load_preset()`

- [ ] **Step 1: Create `flexfetch-core/src/presets.rs`**

```rust
use std::collections::HashMap;
use crate::Config;

pub fn module_group(name: &str) -> Vec<String> {
    match name {
        "flash" => {
            let mut v = module_group("minimal");
            v.push("memory".into());
            v
        }
        "minimal" => vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
        ],
        "full" => Config::default_modules(),
        "dev" => vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "shell".into(),
            "terminal".into(),
        ],
        _ => Config::default_modules(),
    }
}

pub fn builtin_presets() -> HashMap<String, Vec<String>> {
    let mut presets = HashMap::new();
    presets.insert("default".into(), Config::default_modules());
    presets.insert("minimal".into(), module_group("minimal"));
    presets.insert("full".into(), module_group("full"));
    presets.insert("dev".into(), module_group("dev"));
    presets.insert(
        "server".into(),
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "network".into(),
        ],
    );
    presets.insert(
        "laptop".into(),
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "kernel".into(),
            "uptime".into(),
            "cpu".into(),
            "memory".into(),
            "battery".into(),
            "shell".into(),
        ],
    );
    presets.insert(
        "ci".into(),
        vec![
            "os".into(),
            "kernel".into(),
            "cpu".into(),
            "memory".into(),
            "disk".into(),
            "network".into(),
        ],
    );
    presets.insert(
        "neofetch".into(),
        vec![
            "title".into(),
            "separator".into(),
            "os".into(),
            "host".into(),
            "kernel".into(),
            "uptime".into(),
            "packages".into(),
            "shell".into(),
            "de".into(),
            "wm".into(),
            "terminal".into(),
            "cpu".into(),
            "gpu".into(),
            "memory".into(),
            "disk".into(),
            "battery".into(),
            "colors".into(),
        ],
    );
    presets
}

pub fn load_preset(name: &str) -> Vec<String> {
    if let Some(modules) = builtin_presets().get(name) {
        return modules.clone();
    }
    // User presets loaded by CLI (needs config_dir path)
    Config::default_modules()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_group_minimal_has_required_modules() {
        let group = module_group("minimal");
        assert!(group.contains(&"title".to_string()));
        assert!(group.contains(&"os".to_string()));
        assert!(group.contains(&"kernel".to_string()));
    }

    #[test]
    fn builtin_presets_contains_default() {
        let presets = builtin_presets();
        assert!(presets.contains_key("default"));
        assert!(presets.contains_key("minimal"));
        assert!(presets.contains_key("neofetch"));
    }

    #[test]
    fn flash_includes_minimal_plus_memory() {
        let flash = module_group("flash");
        let minimal = module_group("minimal");
        assert!(flash.len() > minimal.len());
        assert!(flash.contains(&"memory".to_string()));
    }
}
```

- [ ] **Step 2: Add to `flexfetch-core/src/lib.rs`**

Add `pub mod presets;` after the existing modules.

- [ ] **Step 3: Run core tests**

Run: `cargo test -p flexfetch-core`
Expected: PASS

- [ ] **Step 4: Update `main.rs` to import from core**

In `flexfetch-cli/src/main.rs`:

- Add `use flexfetch_core::presets;`
- Replace `module_group(name)` with `presets::module_group(name)`
- Replace `builtin_presets()` with `presets::builtin_presets()`
- Replace `load_preset(name)` with a version that also checks user presets dir

- [ ] **Step 5: Run all tests**

Run: `cargo test --workspace`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add flexfetch-core/src/presets.rs flexfetch-core/src/lib.rs flexfetch-cli/src/main.rs
git commit -m "refactor: move preset data from CLI to core for testability"
```

---

## Task 4: Extract logo data to separate module

**Files:**

- Create: `flexfetch-core/src/logo_data.rs`
- Modify: `flexfetch-core/src/logo.rs` (remove const data, import from logo_data)
- Modify: `flexfetch-core/src/lib.rs` (add `pub mod logo_data`)

**Interfaces:**

- Consumes: `Logo` struct (existing)
- Produces: `logo_data::GENERIC_LOGO`, `logo_data::ARCH_LOGO`, etc.

- [ ] **Step 1: Create `flexfetch-core/src/logo_data.rs`**

Move all const Logo definitions from `logo.rs` (lines 220-937) to this new file. The file should contain:

```rust
use crate::logo::Logo;

const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
const WHITE: &str = "\x1b[37m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const YELLOW: &str = "\x1b[33m";

pub const GENERIC_LOGO: Logo = Logo { ... };
pub const ARCH_LOGO: Logo = Logo { ... };
// ... all other logos
```

- [ ] **Step 2: Update `logo.rs` to import from `logo_data`**

Replace all const definitions with imports:

```rust
use crate::logo_data::*;
```

Remove the color constants (CYAN, RESET, etc.) from logo.rs since they're now in logo_data.

- [ ] **Step 3: Add to `lib.rs`**

Add `pub mod logo_data;` to `flexfetch-core/src/lib.rs`.

- [ ] **Step 4: Run tests**

Run: `cargo test --workspace`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add flexfetch-core/src/logo_data.rs flexfetch-core/src/logo.rs flexfetch-core/src/lib.rs
git commit -m "refactor: extract logo const data to logo_data.rs (717 lines)"
```

---

## Task 5: Add tests for `theme::resolve`

**Files:**

- Modify: `flexfetch-core/src/theme.rs` (add #[cfg(test)] mod tests)

**Interfaces:**

- Consumes: `resolve()`, `find_preset()`, `resolve_ansi()`, `preset_names()`
- Produces: unit tests

- [ ] **Step 1: Add test module to `theme.rs`**

At the end of `flexfetch-core/src/theme.rs`, add:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(theme: &str) -> Config {
        let mut config = Config::default_for_testing();
        config.display.theme = Some(theme.to_string());
        config
    }

    #[test]
    fn resolve_default_returns_catppuccin() {
        let config = test_config("");
        let theme = resolve(&config);
        // Default should have non-empty title color
        assert!(!theme.title.is_empty());
    }

    #[test]
    fn resolve_catppuccin() {
        let config = test_config("catppuccin");
        let theme = resolve(&config);
        assert!(!theme.title.is_empty());
        assert!(!theme.keys.is_empty());
    }

    #[test]
    fn resolve_random_returns_valid_theme() {
        let config = test_config("random");
        let theme = resolve(&config);
        assert!(!theme.title.is_empty());
    }

    #[test]
    fn find_preset_returns_some_for_known() {
        assert!(find_preset("catppuccin").is_some());
        assert!(find_preset("dracula").is_some());
    }

    #[test]
    fn find_preset_returns_none_for_unknown() {
        assert!(find_preset("nonexistent_theme").is_none());
    }

    #[test]
    fn preset_names_contains_catppuccin() {
        let names = preset_names();
        assert!(names.contains(&"catppuccin"));
    }

    #[test]
    fn resolve_ansi_named_colors() {
        assert_eq!(resolve_ansi("red"), "\x1b[31m");
        assert_eq!(resolve_ansi("green"), "\x1b[32m");
        assert_eq!(resolve_ansi("blue"), "\x1b[34m");
    }

    #[test]
    fn resolve_ansi_passthrough_escape() {
        let esc = "\x1b[38;2;255;128;0m";
        assert_eq!(resolve_ansi(esc), esc);
    }

    #[test]
    fn resolve_ansi_unknown_returns_empty() {
        assert_eq!(resolve_ansi("notacolor"), "");
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test -p flexfetch-core theme`
Expected: All tests PASS

- [ ] **Step 3: Commit**

```bash
git add flexfetch-core/src/theme.rs
git commit -m "test: add unit tests for theme::resolve and theme::resolve_ansi"
```

---

## Task 6: Add tests for `logo::visible_len` and `logo::detect`

**Files:**

- Modify: `flexfetch-core/src/logo.rs` (add #[cfg(test)] mod tests)

**Interfaces:**

- Consumes: `visible_len()`, `detect()`, `Logo` struct
- Produces: unit tests

- [ ] **Step 1: Add test module to `logo.rs`**

At the end of `flexfetch-core/src/logo.rs`, add:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_len_plain_text() {
        assert_eq!(visible_len("hello"), 5);
        assert_eq!(visible_len(""), 0);
    }

    #[test]
    fn visible_len_ansi_codes_not_counted() {
        // \x1b[31m = 4 bytes, not visible
        assert_eq!(visible_len("\x1b[31mred\x1b[0m"), 3);
    }

    #[test]
    fn visible_len_osc_hyperlink() {
        // OSC: \x1b]8;;url\x1b\\ — URL should not be counted
        let input = "\x1b]8;;https://example.com\x1b\\click here\x1b]8;;\x1b\\";
        assert_eq!(visible_len(input), 10); // "click here"
    }

    #[test]
    fn detect_returns_static_logo() {
        let logo = detect("arch");
        assert!(!logo.lines.is_empty());
    }

    #[test]
    fn detect_unknown_distro_returns_generic() {
        let logo = detect("unknown_distro_xyz");
        assert!(!logo.lines.is_empty());
        // Should be either GENERIC_LOGO or MACOS_LOGO (on macOS)
        assert!(logo.lines.len() > 0);
    }

    #[test]
    fn logo_struct_has_matching_line_color_counts() {
        // Every logo should have either 1 color (applied to all lines) or
        // same count as lines
        let logos = [&ARCH_LOGO, &UBUNTU_LOGO, &FEDORA_LOGO, &GENERIC_LOGO];
        for logo in &logos {
            assert!(
                logo.colors.len() == 1 || logo.colors.len() == logo.lines.len(),
                "Logo colors/lines mismatch: {} colors vs {} lines",
                logo.colors.len(),
                logo.lines.len()
            );
        }
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test -p flexfetch-core logo`
Expected: All tests PASS

- [ ] **Step 3: Commit**

```bash
git add flexfetch-core/src/logo.rs
git commit -m "test: add unit tests for logo::visible_len and logo::detect"
```

---

## Task 7: Final verification

- [ ] **Step 1: Run full test suite**

Run: `cargo test --workspace`
Expected: All tests PASS

- [ ] **Step 2: Run clippy**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings

- [ ] **Step 3: Verify no duplicate `info_value_summary`**

Run: `rg "fn info_value_summary" --type rust`
Expected: No results

- [ ] **Step 4: Verify presets compile from core**

Run: `cargo build -p flexfetch-core --features "default"`
Expected: Build succeeds

- [ ] **Step 5: Final commit if any fixes needed**

```bash
git add -A
git commit -m "chore: architecture deepening phase 2 complete"
```
