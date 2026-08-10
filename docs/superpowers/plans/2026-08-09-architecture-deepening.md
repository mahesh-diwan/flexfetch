# Architecture Deepening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Collapse the main.rs monolith into focused modules, fix Config merge boilerplate, unify MODULE_CATALOG + ModuleRegistry, deepen template.rs, add export adapter seams, and add test seams to collector modules.

**Architecture:** Extract 4 modules from main.rs (`cli::dispatch`, `config::load`, `registry::resolve`, `render::output`). Fix DisplayConfig merge by replacing per-field merge with iterator-based merge. Unify MODULE_CATALOG as the single source of truth for registry building. Deepen template.rs with internal seams (compile, group, align, logo). Create export module with 3 grouped adapters (image, text, structured). Add `read_file()` to Context for testable collectors.

**Tech Stack:** Rust 1.75+, clap (CLI), serde/toml (config), Tera (templates), rayon (parallel), criterion (benchmarks)

## Global Constraints

- All existing tests must pass after each task
- No breaking changes to public API (flexfetch-core crate)
- Feature flags preserved (tera, parallel, live, lua, wasm-plugins, qr, music, image-logos, auto-theme)
- Platform branching preserved (#[cfg(target_os = "...")])
- Minimum Rust version: 1.75

---

## File Structure

| File                                    | Responsibility                                               |
| --------------------------------------- | ------------------------------------------------------------ |
| `flexfetch-cli/src/main.rs`             | Thin dispatcher — CLI parse, subcommand dispatch, watch loop |
| `flexfetch-cli/src/cli_dispatch.rs`     | **NEW** — subcommand/flag dispatch logic                     |
| `flexfetch-cli/src/config_load.rs`      | **NEW** — config loading + CLI overrides                     |
| `flexfetch-cli/src/registry_resolve.rs` | **NEW** — module resolution + preset loading                 |
| `flexfetch-cli/src/render_output.rs`    | **NEW** — render dispatch + export handling                  |
| `flexfetch-core/src/config.rs`          | Modified — DisplayConfig merge via iterator                  |
| `flexfetch-core/src/module.rs`          | Modified — MODULE_CATALOG as registry source                 |
| `flexfetch-core/src/module_registry.rs` | Modified — auto-build from catalog                           |
| `flexfetch-core/src/template.rs`        | Modified — internal seams (compile, group, align, logo)      |
| `flexfetch-core/src/export.rs`          | Modified — grouped adapters (image, text, structured)        |
| `flexfetch-core/src/context.rs`         | Modified — add `read_file()` abstraction                     |

---

## Task 1: Add `read_file()` to Context

**Files:**

- Modify: `flexfetch-core/src/context.rs:7-30`
- Test: `flexfetch-core/src/context.rs` (inline)

**Interfaces:**

- Consumes: `Context::new()` signature (unchanged)
- Produces: `ctx.read_file(path) -> Result<String, std::io::Error>`

- [ ] **Step 1: Add read_file method to Context**

```rust
// In flexfetch-core/src/context.rs, add to impl Context:

/// Read a file through the context abstraction. Modules should use this
/// instead of std::fs::read_to_string to enable testing with mock data.
pub fn read_file(&self, path: impl AsRef<std::path::Path>) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
```

- [ ] **Step 2: Add test with mock**

```rust
// In flexfetch-core/src/context.rs, add to #[cfg(test)] mod tests:

#[test]
fn read_file_returns_content() {
    let dir = std::env::temp_dir().join(format!("ff-ctx-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("test.txt");
    std::fs::write(&path, "hello").unwrap();
    let ctx = Context::new(dir.clone(), dir.clone(), false, Default::default());
    assert_eq!(ctx.read_file(&path).unwrap(), "hello");
    let _ = std::fs::remove_dir_all(&dir);
}
```

- [ ] **Step 3: Run test**

Run: `cargo test -p flexfetch-core context::tests::read_file_returns_content`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add flexfetch-core/src/context.rs
git commit -m "feat(core): add read_file() to Context for testable collectors"
```

---

## Task 2: Migrate collector modules to use ctx.read_file()

**Files:**

- Modify: `flexfetch-core/src/modules/os.rs`
- Modify: `flexfetch-core/src/modules/host.rs`
- Modify: `flexfetch-core/src/modules/kernel.rs`
- Modify: `flexfetch-core/src/modules/cpu.rs`
- Modify: `flexfetch-core/src/modules/memory.rs`
- Modify: `flexfetch-core/src/modules/disk.rs`
- Modify: `flexfetch-core/src/modules/battery.rs`
- Modify: `flexfetch-core/src/modules/shell.rs`
- Modify: `flexfetch-core/src/modules/terminal.rs`
- Modify: `flexfetch-core/src/modules/uptime.rs`
- Modify: `flexfetch-core/src/modules/swap.rs`
- Modify: `flexfetch-core/src/modules/temperature.rs`
- Modify: `flexfetch-core/src/modules/network.rs`
- Modify: `flexfetch-core/src/modules/gpu.rs`
- Modify: `flexfetch-core/src/modules/display.rs`
- Modify: `flexfetch-core/src/modules/resolution.rs`
- Modify: `flexfetch-core/src/modules/processes.rs`
- Modify: `flexfetch-core/src/modules/dns.rs`
- Modify: `flexfetch-core/src/modules/locale.rs`
- Modify: `flexfetch-core/src/modules/packages.rs`
- Modify: `flexfetch-core/src/modules/de.rs`
- Modify: `flexfetch-core/src/modules/wm.rs`

**Interfaces:**

- Consumes: `ctx.read_file(path)` from Task 1
- Produces: (no new public interface — mechanical replacement)

- [ ] **Step 1: Replace fs::read_to_string with ctx.read_file in os.rs**

In `flexfetch-core/src/modules/os.rs`, find all `std::fs::read_to_string` or `std::fs::read_to_string` calls and replace with `ctx.read_file`. Example:

```rust
// Before:
let content = std::fs::read_to_string("/etc/os-release").unwrap_or_default();

// After:
let content = ctx.read_file("/etc/os-release").unwrap_or_default();
```

- [ ] **Step 2: Repeat for all 22 module files**

For each module file in `flexfetch-core/src/modules/`, replace `std::fs::read_to_string(path)` with `ctx.read_file(path)`. The pattern is mechanical — find the call, replace it.

- [ ] **Step 3: Run full test suite**

Run: `cargo test -p flexfetch-core`
Expected: All existing tests pass

- [ ] **Step 4: Run clippy**

Run: `cargo clippy -p flexfetch-core -- -D warnings`
Expected: No warnings

- [ ] **Step 5: Commit**

```bash
git add flexfetch-core/src/modules/
git commit -m "feat(core): migrate collector modules to ctx.read_file()"
```

---

## Task 3: Fix DisplayConfig merge boilerplate

**Files:**

- Modify: `flexfetch-core/src/config.rs:50-144` (DisplayConfig struct)
- Modify: `flexfetch-core/src/config.rs:422-504` (merge_config function)

**Interfaces:**

- Consumes: DisplayConfig fields (unchanged)
- Produces: merge_config() still works, but uses iterator-based merge

- [ ] **Step 1: Add merge helper to DisplayConfig**

```rust
// In flexfetch-core/src/config.rs, add to impl DisplayConfig:

impl DisplayConfig {
    /// Merge two DisplayConfig instances. `override_config` values win over
    /// `base` values, except for Option fields where None means "keep base".
    fn merge(base: DisplayConfig, override_config: DisplayConfig) -> DisplayConfig {
        DisplayConfig {
            separator: override_config.separator,
            key_width: override_config.key_width,
            theme: override_config.theme.or(base.theme),
            color_title: override_config.color_title.or(base.color_title),
            color_keys: override_config.color_keys.or(base.color_keys),
            color_values: override_config.color_values.or(base.color_values),
            color_sep: override_config.color_sep.or(base.color_sep),
            gradient: override_config.gradient || base.gradient,
            gradient_colors: override_config.gradient_colors.or(base.gradient_colors),
            logo_mode: override_config.logo_mode,
            gradient_title: override_config.gradient_title,
            progress_bars: override_config.progress_bars,
            box_style: override_config.box_style,
            palette_style: override_config.palette_style,
            frame: override_config.frame,
            logo_gradient: override_config.logo_gradient,
            sections: override_config.sections,
            // Icons: override wins unconditionally
            icon_os: override_config.icon_os,
            icon_kernel: override_config.icon_kernel,
            icon_host: override_config.icon_host,
            icon_uptime: override_config.icon_uptime,
            icon_locale: override_config.icon_locale,
            icon_cpu: override_config.icon_cpu,
            icon_gpu: override_config.icon_gpu,
            icon_memory: override_config.icon_memory,
            icon_swap: override_config.icon_swap,
            icon_disk: override_config.icon_disk,
            icon_network: override_config.icon_network,
            icon_interface: override_config.icon_interface,
            icon_resolution: override_config.icon_resolution,
            icon_battery: override_config.icon_battery,
            icon_processes: override_config.icon_processes,
            icon_end: override_config.icon_end,
            icon_temp: override_config.icon_temp,
        }
    }
}
```

- [ ] **Step 2: Simplify merge_config to use DisplayConfig::merge**

```rust
// In flexfetch-core/src/config.rs, replace merge_config function:

fn merge_config(base: Config, override_config: Config) -> Config {
    Config {
        version: override_config.version,
        modules: if override_config.modules != Config::default_modules() {
            override_config.modules
        } else {
            base.modules
        },
        template: if override_config.template != "default" {
            override_config.template
        } else {
            base.template
        },
        plugins_dir: override_config.plugins_dir.or(base.plugins_dir),
        display: DisplayConfig::merge(base.display, override_config.display),
        custom: if !override_config.custom.is_empty() {
            override_config.custom
        } else {
            base.custom
        },
        modules_config: if !override_config.modules_config.is_empty() {
            override_config.modules_config
        } else {
            base.modules_config
        },
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p flexfetch-core`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add flexfetch-core/src/config.rs
git commit -m "refactor(core): extract DisplayConfig::merge() to reduce boilerplate"
```

---

## Task 4: Unify MODULE_CATALOG as registry source of truth

**Files:**

- Modify: `flexfetch-core/src/module.rs:73-315` (MODULE_CATALOG)
- Modify: `flexfetch-core/src/module_registry.rs:28-188` (ModuleRegistry::build)

**Interfaces:**

- Consumes: MODULE_CATALOG entries
- Produces: ModuleRegistry auto-builds from catalog

- [ ] **Step 1: Add builder_fn to MODULE_CATALOG entries**

```rust
// In flexfetch-core/src/module.rs, add a builder function field to ModuleInfo:

pub struct ModuleInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub section: &'static str,
    pub is_static: bool,
    pub builder: fn() -> Box<dyn Module>,
}
```

- [ ] **Step 2: Update MODULE_CATALOG entries with builder functions**

```rust
// Example entry in MODULE_CATALOG:
ModuleInfo {
    name: "os",
    description: "Operating system",
    section: "system",
    is_static: true,
    builder: || Box::new(crate::modules::os::OsModule),
},
```

- [ ] **Step 3: Update ModuleRegistry::build to use catalog**

```rust
// In flexfetch-core/src/module_registry.rs, replace ModuleRegistry::build:

fn build() -> Self {
    let builders = MODULE_CATALOG
        .iter()
        .map(|m| (m.name, (m.builder)()))
        .collect();
    ModuleRegistry { builders }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p flexfetch-core`
Expected: All tests pass

- [ ] **Step 5: Run clippy**

Run: `cargo clippy -p flexfetch-core -- -D warnings`
Expected: No warnings

- [ ] **Step 6: Commit**

```bash
git add flexfetch-core/src/module.rs flexfetch-core/src/module_registry.rs
git commit -m "refactor(core): unify MODULE_CATALOG as single source of truth"
```

---

## Task 5: Extract main.rs into focused modules

**Files:**

- Modify: `flexfetch-cli/src/main.rs`
- Create: `flexfetch-cli/src/cli_dispatch.rs`
- Create: `flexfetch-cli/src/config_load.rs`
- Create: `flexfetch-cli/src/registry_resolve.rs`
- Create: `flexfetch-cli/src/render_output.rs`

**Interfaces:**

- Consumes: `Cli` struct, `Config`, `Context`, `ModuleRegistry`
- Produces: Thin main.rs dispatcher

- [ ] **Step 1: Create cli_dispatch.rs**

```rust
// flexfetch-cli/src/cli_dispatch.rs
use clap::Parser;
use flexfetch_cli::{Cli, Commands};
use flexfetch_core::get_cache_dir;

/// Handle subcommands that run before config load.
pub fn handle_subcommands(cli: &Cli) -> bool {
    if let Some(command) = &cli.command {
        match command {
            #[cfg(feature = "completions")]
            Commands::Completions { shell } => {
                use clap::CommandFactory;
                let mut cmd = Cli::command();
                clap_complete::generate(*shell, &mut cmd, "flexfetch", &mut std::io::stdout());
            }
            Commands::Plugin { action } => crate::registry::run(action),
        }
        return true;
    }
    false
}

/// Handle flags that run before config load (--gen-config, --list-modules, etc.)
pub fn handle_preflags(cli: &Cli) -> bool {
    if cli.gen_config {
        crate::generate_config();
        return true;
    }
    if cli.list_modules {
        crate::list_modules();
        return true;
    }
    if cli.list_presets {
        crate::list_presets();
        return true;
    }
    if cli.list_themes {
        for name in flexfetch_core::theme::preset_names() {
            println!("{name}");
        }
        return true;
    }
    if let Some(ref shell) = cli.hook {
        crate::tools::print_hook(shell);
        return true;
    }
    if cli.update {
        crate::tools::self_update();
        return true;
    }
    if cli.update_db {
        match flexfetch_core::hardware_db::refresh() {
            Ok(msg) => println!("{msg}"),
            Err(e) => {
                eprintln!("update-db: {e}");
                std::process::exit(1);
            }
        }
        return true;
    }
    false
}
```

- [ ] **Step 2: Create config_load.rs**

```rust
// flexfetch-cli/src/config_load.rs
use flexfetch_core::{Config, Context};
use std::path::PathBuf;

/// Load config and create context. Returns (config, ctx, config_dir, cache_dir).
pub fn load(
    config_path: Option<&std::path::Path>,
    flash: bool,
    debug: bool,
    custom: std::collections::HashMap<String, flexfetch_core::config::CustomModule>,
) -> (Config, Context, PathBuf, PathBuf) {
    let config_dir = crate::tools::config_dir();
    let cache_dir = flexfetch_core::get_cache_dir();
    let config = if flash {
        Config::default_for_testing()
    } else {
        Config::load(config_path).unwrap_or_else(|_| Config::default_for_testing())
    };
    let ctx = Context::new(config_dir.clone(), cache_dir.clone(), debug, custom);
    (config, ctx, config_dir, cache_dir)
}
```

- [ ] **Step 3: Create registry_resolve.rs**

```rust
// flexfetch-cli/src/registry_resolve.rs
use flexfetch_cli::Cli;
use flexfetch_core::{Config, ModuleRegistry};

/// Resolve module list from CLI flags/presets/config.
pub fn resolve(cli: &Cli, config: &Config) -> Vec<String> {
    crate::resolve_modules(cli, config)
}

/// Get the static module registry.
pub fn registry() -> &'static ModuleRegistry {
    ModuleRegistry::get()
}
```

- [ ] **Step 4: Create render_output.rs**

```rust
// flexfetch-cli/src/render_output.rs
use flexfetch_cli::Cli;
use flexfetch_core::{Config, SystemInfo};

/// Render output in the requested format.
pub fn render(info: &SystemInfo, config: &Config, cli: &Cli, ssh: bool) {
    crate::render_output(info, config, cli, ssh)
}

/// Handle export to file.
pub fn export(
    info: &SystemInfo,
    config: &Config,
    format: &str,
    output: Option<&std::path::Path>,
) -> bool {
    crate::handle_export(info, config, format, output)
}
```

- [ ] **Step 5: Rewrite main.rs as thin dispatcher**

```rust
// flexfetch-cli/src/main.rs — rewritten as thin dispatcher
use clap::Parser;
use flexfetch_cli::Cli;
use flexfetch_core::get_cache_dir;

mod live;
mod qr;
mod plugins;
mod registry;
mod ssh;
mod tools;
mod wizard;
mod telemetry;
mod cli_dispatch;
mod config_load;
mod registry_resolve;
mod render_output;

fn main() {
    telemetry::install_panic_hook(&get_cache_dir());
    let t_cold_start = std::time::Instant::now();

    // Handle --version before clap
    if std::env::args().any(|a| a == "--version" || a == "-V") {
        // ... (version display code unchanged)
        return;
    }

    let cli = Cli::parse();
    let cli = if telemetry::debug_enabled() {
        Cli { debug: true, ..cli }
    } else {
        cli
    };

    // Subcommands and pre-flags
    if cli_dispatch::handle_subcommands(&cli) { return; }
    if cli_dispatch::handle_preflags(&cli) { return; }

    // Import QR config
    if cli.import_qr.is_some() {
        // ... (QR import code unchanged)
    }

    // Load config
    let config_path = cli.config.as_ref().map(std::path::Path::new);
    let (mut config, mut ctx, config_dir, cache_dir) =
        config_load::load(config_path, cli.flash, cli.debug, config.custom.clone());

    // Post-config flags
    if cli.doctor { tools::run_doctor(&ctx); return; }
    if cli.bug_report { print!("{}", telemetry::generate_bug_report(&ctx, &config)); return; }
    if cli.qr { /* ... unchanged ... */ }
    if cli.live { /* ... unchanged ... */ }

    // Module resolution
    let mut modules = registry_resolve::resolve(&cli, &config);
    apply_cli_overrides(&cli, &mut config, pipe_mode);
    let registry = registry_resolve::registry();
    let template_content = TeraEngine::default_template_content();

    // ... rest of dispatch unchanged, using render_output::render() ...
}
```

- [ ] **Step 6: Run tests**

Run: `cargo test -p flexfetch-cli`
Expected: All tests pass

- [ ] **Step 7: Run clippy**

Run: `cargo clippy -p flexfetch-cli -- -D warnings`
Expected: No warnings

- [ ] **Step 8: Commit**

```bash
git add flexfetch-cli/src/
git commit -m "refactor(cli): extract main.rs into focused modules

- cli_dispatch: subcommand and pre-flag handling
- config_load: config loading + context creation
- registry_resolve: module resolution + preset loading
- render_output: render dispatch + export handling"
```

---

## Task 6: Deepen template.rs with internal seams

**Files:**

- Modify: `flexfetch-core/src/template.rs`

**Interfaces:**

- Consumes: `TeraEngine::render(info, config)` (unchanged)
- Produces: Internal seams for compile, group, align, logo

- [ ] **Step 1: Extract template compilation seam**

```rust
// In flexfetch-core/src/template.rs, extract a compile function:

/// Compile a Tera template string into a Tera instance.
pub fn compile_template(template_str: &str) -> Result<Tera, tera::Error> {
    let mut tera = Tera::default();
    tera.add_raw_template("main", template_str)?;
    Ok(tera)
}
```

- [ ] **Step 2: Extract section grouping seam**

```rust
// In flexfetch-core/src/template.rs, extract a group function:

/// Group InfoValue entries by section (System, Hardware, etc.).
pub fn group_sections(entries: &[(String, InfoValue)], config: &Config) -> Vec<(String, Vec<(String, InfoValue)>)> {
    // Move the existing section grouping logic here
    // ...
}
```

- [ ] **Step 3: Extract key alignment seam**

```rust
// In flexfetch-core/src/template.rs, extract an align function:

/// Align keys in a section for consistent column width.
pub fn align_keys(entries: &[(String, InfoValue)], config: &Config) -> Vec<(String, InfoValue)> {
    // Move the existing alignment logic here
    // ...
}
```

- [ ] **Step 4: Extract logo rendering seam**

```rust
// In flexfetch-core/src/template.rs, extract a logo function:

/// Render the ASCII logo for the given theme.
pub fn render_logo(config: &Config, theme: &ThemeStrings) -> Option<String> {
    // Move the existing logo rendering logic here
    // ...
}
```

- [ ] **Step 5: Add unit tests for each seam**

```rust
// In flexfetch-core/src/template.rs, add tests:

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_template_valid() {
        let result = compile_template("Hello {{ name }}");
        assert!(result.is_ok());
    }

    #[test]
    fn compile_template_invalid() {
        let result = compile_template("Hello {% invalid %}");
        assert!(result.is_err());
    }

    #[test]
    fn group_sections_basic() {
        let entries = vec![
            ("os".to_string(), InfoValue::Scalar("Linux".to_string())),
            ("cpu".to_string(), InfoValue::Scalar("Intel".to_string())),
        ];
        let config = Config::default_for_testing();
        let groups = group_sections(&entries, &config);
        assert!(!groups.is_empty());
    }
}
```

- [ ] **Step 6: Run tests**

Run: `cargo test -p flexfetch-core template::tests`
Expected: All tests pass

- [ ] **Step 7: Commit**

```bash
git add flexfetch-core/src/template.rs
git commit -m "refactor(core): extract internal seams from template.rs

- compile_template: Tera compilation
- group_sections: section grouping
- align_keys: key alignment
- render_logo: logo rendering"
```

---

## Task 7: Create export adapters with grouping

**Files:**

- Modify: `flexfetch-core/src/export.rs`

**Interfaces:**

- Consumes: `SystemInfo`, `Config`
- Produces: Export adapters (image, text, structured)

- [ ] **Step 1: Group image exports**

```rust
// In flexfetch-core/src/export.rs, group image exports:

pub mod image {
    use super::*;

    pub fn export_svg(info: &SystemInfo, config: &Config) -> Result<String, crate::Error> {
        // Move existing SVG export logic here
        super::export_svg(info, config)
    }

    pub fn export_png(info: &SystemInfo, config: &Config, path: &std::path::Path) -> Result<(), crate::Error> {
        // Move existing PNG export logic here
        super::export_png(info, config, path)
    }
}
```

- [ ] **Step 2: Group text exports**

```rust
pub mod text {
    use super::*;

    pub fn export_markdown(info: &SystemInfo, config: &Config) -> Result<String, crate::Error> {
        super::export_markdown(info, config)
    }

    pub fn export_csv(info: &SystemInfo) -> Result<String, crate::Error> {
        super::export_csv(info)
    }
}
```

- [ ] **Step 3: Group structured exports**

```rust
pub mod structured {
    use super::*;

    pub fn export_prometheus(info: &SystemInfo) -> Result<String, crate::Error> {
        super::export_prometheus(info)
    }

    pub fn export_github(info: &SystemInfo) -> Result<String, crate::Error> {
        super::export_github(info)
    }

    pub fn export_ansible(info: &SystemInfo) -> Result<String, crate::Error> {
        super::export_ansible(info)
    }

    pub fn export_terraform(info: &SystemInfo) -> Result<String, crate::Error> {
        super::export_terraform(info)
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p flexfetch-core`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add flexfetch-core/src/export.rs
git commit -m "refactor(core): group export adapters (image, text, structured)"
```

---

## Task 8: Verify all tests pass

**Files:** None (verification only)

- [ ] **Step 1: Run full test suite**

Run: `cargo test --workspace`
Expected: All tests pass

- [ ] **Step 2: Run clippy on workspace**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings

- [ ] **Step 3: Build release**

Run: `cargo build --release`
Expected: Builds successfully

- [ ] **Step 4: Test CLI manually**

Run: `./target/release/flexfetch`
Expected: Displays system info correctly

- [ ] **Step 5: Test --flash**

Run: `./target/release/flexfetch --flash`
Expected: Fast output with minimal modules

- [ ] **Step 6: Test --export svg**

Run: `./target/release/flexfetch --export svg`
Expected: Writes flexfetch.svg

- [ ] **Step 7: Commit final verification**

```bash
git add -A
git commit -m "verify: all tests pass after architecture deepening"
```

---

## Summary

| Task | Candidate | What changes                                |
| ---- | --------- | ------------------------------------------- |
| 1    | 6         | Add `read_file()` to Context                |
| 2    | 6         | Migrate 22 modules to use `ctx.read_file()` |
| 3    | 2         | Extract `DisplayConfig::merge()`            |
| 4    | 3         | Unify MODULE_CATALOG as registry source     |
| 5    | 1         | Extract main.rs into 4 modules              |
| 6    | 4         | Deepen template.rs with internal seams      |
| 7    | 5         | Group export adapters                       |
| 8    | —         | Verify everything works                     |

**Execution order:** 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8

Each task is independently testable and committable. Tasks 1-2 are foundational (enable testable collectors). Tasks 3-4 clean up the config/registry coupling. Task 5 is the main structural change. Tasks 6-7 deepen the extracted modules. Task 8 verifies the whole thing.
