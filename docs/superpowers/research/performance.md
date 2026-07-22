# Performance Optimization Research — flexfetch

## Current Architecture Analysis

### What exists today

| Component | Current State | Issue |
|-----------|---------------|-------|
| Template parsing | `Tera::default()` + `add_raw_template()` every run | ~5-15ms wasted on re-parse |
| Module execution | All 20 modules always run via `run_selected()` | Unreferenced modules still execute |
| Parallelism | Rayon `par_iter()` for module collection | Good, already parallel |
| Binary size | 5.1MB (release: lto=true, codegen-units=1, opt-level=z, strip=true, panic=abort) | Maxed out on standard knobs |
| Config loading | TOML parsed from disk every run | ~1-2ms |

### Hot path per run

```
main() → Config::load() → ModuleRegistry::new() → run_selected() → TeraEngine::new_default() → render()
```

The `TeraEngine::new_default()` call creates a fresh `Tera` instance and parses the template every single invocation. This is the easiest win.

---

## Prioritized Optimizations

### 1. Template Caching via `OnceLock` — HIGH impact, ~5-15ms saved

**Current:** `TeraEngine::new_default()` creates `Tera::default()` and calls `add_raw_template()` every run.

**Fix:** Use `std::sync::OnceLock` to compile the template once, reuse across runs.

```rust
use std::sync::OnceLock;

static CACHED_TERA: OnceLock<Tera> = OnceLock::new();

fn get_tera() -> &'static Tera {
    CACHED_TERA.get_or_init(|| {
        let mut tera = Tera::default();
        tera.add_raw_template("default", include_str!("../../templates/default.tera"))
            .expect("default template is valid");
        tera
    })
}

impl TeraEngine {
    pub fn new_default() -> Self {
        TeraEngine {
            tera: get_tera().clone(),  // Tera::clone is cheap (Arc internally)
            template_name: "default".to_string(),
        }
    }
}
```

**Why this works:** Tera internally uses `Arc` for template storage. Cloning a `Tera` instance is a reference count increment, not a re-parse. The `OnceLock` ensures parsing happens exactly once.

**Estimated savings:** 5-15ms per run (template parsing dominates startup).

---

### 2. Module Skip List — HIGH impact, varies by config

**Current:** `run_selected()` receives the full module list and runs all of them (filtering only `title`/`separator`).

**Actual behavior:** The template already uses `{%- if os -%}` conditionals, so unreferenced modules render to empty. But the modules still *execute* — the OS detection, package counting, GPU probing all run even if the template doesn't use them.

**Fix:** Parse the template for variable references, only execute modules that appear.

```rust
use std::collections::HashSet;

/// Extract module names referenced in a Tera template.
/// Scans for `{module_name` patterns (variable access).
fn extract_template_modules(template_str: &str) -> HashSet<String> {
    let mut modules = HashSet::new();
    let mut chars = template_str.char_indices().peekable();
    
    while let Some((i, ch)) = chars.next() {
        if ch == '{' {
            // Skip {{ and {% constructs
            if chars.peek().map(|(_, c)| *c == '{' || *c == '%').unwrap_or(false) {
                // This is a block/filter, skip to closing %}
                // but we still want to find variable names inside
            }
            // Find the variable name after { or {{
            let start = i + 1;
            // skip whitespace and braces
            let name_start = template_str[start..]
                .trim_start_matches(|c: char| c == '{' || c == '%' || c == '-' || c.is_whitespace())
                .as_ptr() as usize;
            // read until space, |, }, or end
            let name: String = template_str[name_start..]
                .chars()
                .take_while(|c| !c.is_whitespace() && *c != '|' && *c != '}' && *c != '%')
                .collect();
            
            // Filter out Tera keywords and display constants
            let skip = ["if", "endif", "else", "elif", "for", "endfor", 
                        "set", "endset", "block", "endblock", "include",
                        "theme_keys", "theme_reset", "theme_values", 
                        "theme_title", "theme_sep", "display_separator", 
                        "display_key_width"];
            
            if !name.is_empty() && !skip.contains(&name.as_str()) {
                // Strip trailing dots for nested access (e.g., "os.pretty_name" → "os")
                if let Some(base) = name.split('.').next() {
                    if !base.is_empty() {
                        modules.insert(base.to_string());
                    }
                }
            }
        }
    }
    modules
}
```

Then in `run_selected()`:

```rust
pub fn run_selected(&self, selected: &[String], ctx: &Context) -> SystemInfo {
    // Filter to only modules the template references
    let template_modules = extract_template_modules(&ctx.template_content);
    let effective: Vec<&str> = selected.iter()
        .filter(|m| template_modules.is_empty() || template_modules.contains(m.as_str()))
        .map(|s| s.as_str())
        .collect();
    
    // ... existing rayon parallel collection on effective list
}
```

**Impact:** Depends on config. A 5-module template skips 15 modules. Each module is 1-50ms. Could save 50-200ms.

---

### 3. Benchmark Mode — LOW effort, HIGH value for tuning

**Design:** Add `--benchmark` flag that times each module and reports.

```rust
// In main.rs
#[arg(long)]
benchmark: bool,

// In the benchmark path:
if cli.benchmark {
    let start = std::time::Instant::now();
    let config_load = start.elapsed();
    
    let start = std::time::Instant::now();
    let registry = ModuleRegistry::new(&config);
    let registry_init = start.elapsed();
    
    // Time each module individually
    let mut timings: Vec<(&str, std::time::Duration)> = Vec::new();
    for name in &modules {
        if name == "title" || name == "separator" { continue; }
        let start = std::time::Instant::now();
        if let Some((_, builder)) = registry.builders.iter().find(|(n, _)| n == name) {
            let module = builder();
            let _ = module.collect(&ctx);
        }
        timings.push((name, start.elapsed()));
    }
    
    let start = std::time::Instant::now();
    let _ = engine.render(&info, &config);
    let render_time = start.elapsed();
    
    // Report
    eprintln!("--- flexfetch benchmark ---");
    eprintln!("config load:    {config_load:?}");
    eprintln!("registry init:  {registry_init:?}");
    for (name, dur) in &timings {
        eprintln!("  {name:15} {dur:?}");
    }
    eprintln!("template render:{render_time:?}");
    eprintln!("total:          {:?}", start_total.elapsed());
}
```

**Companion:** Use `hyperfine` for before/after comparisons:
```bash
hyperfine --warmup 3 './target/release/flexfetch --benchmark'
```

---

### 4. Binary Size — LOW impact (already optimized)

Current profile is already maxed:

```toml
[profile.release]
lto = true          # Fat LTO — already on
codegen-units = 1   # Single unit — already on
opt-level = "z"     # Size-optimized — already on
strip = true        # Strip symbols — already on
panic = "abort"     # No unwinding — already on
```

**Remaining options (diminishing returns):**

| Technique | Expected Savings | Effort |
|-----------|-----------------|--------|
| `opt-level = "s"` instead of `"z"` | 0-5% (sometimes smaller!) | Change one line |
| Profile-Guided Optimization (PGO) | 5-15% | Medium — instrument, run, recompile |
| `cargo-bloat --crates` analysis | Identify bloat | Run tool |
| Remove unused deps via `cargo-udeps` | 10-50KB per dep | Audit |
| Conditional compilation for modules | 20-100KB | High — feature flags per module |

**PGO approach:**
```bash
# Step 1: Build instrumented
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release

# Step 2: Run to collect profiles
./target/release/flexfetch
./target/release/flexfetch -c custom.jsonc

# Step 3: Merge profiles
llvm-profdata merge -output=/tmp/pgo-data/merged.profdata /tmp/pgo-data/*.profraw

# Step 4: Build optimized
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release
```

---

### 5. Module Registry — Static Dispatch — MEDIUM impact

**Current:** `ModuleRegistry` stores `Vec<(&'static str, ModuleBuilder)>` with closures. Module lookup is `find()` — O(n) per module.

**fastfetch approach:** Alphabetically indexed array `ffModuleInfos[26]` — O(1) lookup by first letter.

**Lazy fix:** Sort the builders vec and use binary search, or use a `HashMap`. But with 20 modules, linear scan is ~20 comparisons — negligible.

**Skip this.** Not a bottleneck at 20 modules.

---

### 6. Flashfetch Pattern — Compile-time Module Selection

fastfetch has a `flashfetch.c` that hardcodes module calls for maximum speed — no registry lookup, no config parsing.

**For flexfetch:** Could generate a `flashfetch.rs` that directly calls the fastest path:

```rust
// Generated or hand-written for the default config
pub fn flash_render() -> String {
    let ctx = Context::default();
    let os = crate::modules::os::OsModule.collect(&ctx);
    let cpu = crate::modules::cpu::CpuModule.collect(&ctx);
    // ... only the modules in default template
    // Direct string formatting, no Tera
}
```

**Impact:** Could cut default config time by 30-50%. But adds maintenance burden. Defer until benchmarks show template rendering is not the bottleneck.

---

## Summary: Top 3 by Impact

| Rank | Optimization | Impact | Effort | Files Changed |
|------|-------------|--------|--------|---------------|
| 1 | Template caching via `OnceLock` | HIGH (5-15ms) | Low | `template.rs` |
| 2 | Module skip list | HIGH (50-200ms) | Medium | `module_registry.rs`, `template.rs` |
| 3 | Benchmark mode | Enables all other optimization | Low | `main.rs` |

**Deferred:** PGO (needs profiling infrastructure first), flashfetch pattern (needs benchmarks to justify).

---

## Implementation Order

1. Add `--benchmark` flag — measure current state
2. Template caching — trivial OnceLock change
3. Module skip list — parse template, filter module list
4. Run benchmarks, compare
5. If still slow: PGO or flashfetch pattern

---

## References

- fastfetch architecture: https://deepwiki.com/fastfetch-cli/fastfetch/2-core-application-architecture
- fastfetch module system: https://deepwiki.com/fastfetch-cli/fastfetch/2.3-module-system-architecture
- Tera OnceLock pattern: https://www.shuttle.dev/blog/2024/11/29/the-essence-of-templating-with-tera
- Rust binary size optimization: https://microsoft.github.io/RustTraining/engineering-book/ch07-release-profiles-and-binary-size.html
- PGO in Rust: https://doc.rust-lang.org/stable/rustc/profile-guided-optimization.html
- hyperfine: https://github.com/sharkdp/hyperfine
- criterion.rs: https://github.com/criterion-rs/criterion.rs
