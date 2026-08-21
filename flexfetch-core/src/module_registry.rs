use crate::{Context, InfoValue, Module, SystemInfo, MODULE_CATALOG};
use std::collections::HashSet;
use std::sync::{Arc, OnceLock};
#[cfg(not(feature = "parallel"))]
use std::time::Duration;
use std::time::Instant;

/// Wall-clock budget for one module's collection on the sequential path.
/// A hung sensor (DBus probe, HTTP fetch, custom shell command) degrades to
/// a "timeout" value instead of stalling the fetch. The collection thread
/// detaches on timeout and dies with the process. The parallel (rayon) path
/// collects inline instead — blocking a rayon worker on a per-module thread
/// serializes the whole batch and costs ~6x cold start; HTTP/DBus modules
/// carry their own internal timeouts there.
#[cfg(not(feature = "parallel"))]
const MODULE_TIMEOUT: Duration = Duration::from_millis(2000);

type SharedContext = Arc<Context>;

/// Collect one module, recording `--stat` wall-clock timing when enabled.
fn collect_one(
    name: &str,
    builder: fn() -> Box<dyn Module>,
    ctx: &SharedContext,
) -> crate::Result<InfoValue> {
    let timed = ctx.stat;
    let start = Instant::now();
    #[cfg(feature = "parallel")]
    let result = builder().collect(ctx);
    #[cfg(not(feature = "parallel"))]
    let result = collect_with_timeout(builder, ctx);
    if timed {
        if let Ok(mut t) = ctx.timings.lock() {
            t.push((name.to_string(), start.elapsed().as_micros() as u64));
        }
    }
    result
}

/// Sequential path: collect on a detached thread with a timeout so a single
/// hung module can never stall the fetch.
#[cfg(not(feature = "parallel"))]
fn collect_with_timeout(
    builder: fn() -> Box<dyn Module>,
    ctx: &SharedContext,
) -> crate::Result<InfoValue> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx2 = Arc::clone(ctx);
    std::thread::spawn(move || {
        let _ = tx.send(builder().collect(&ctx2));
    });
    match rx.recv_timeout(MODULE_TIMEOUT) {
        Ok(r) => r,
        Err(_) => Ok(InfoValue::scalar("timeout")),
    }
}

type ModuleBuilder = Box<dyn Module>;

/// Derive template module names from the catalog — single source of truth.
fn extract_template_modules(template_str: &str) -> HashSet<String> {
    MODULE_CATALOG
        .iter()
        .filter(|m| template_str.contains(m.name))
        .map(|m| m.name.to_string())
        .collect()
}

pub struct ModuleRegistry {
    builders: Vec<(&'static str, ModuleBuilder)>,
}

// Cached registry - built once, reused forever
static REGISTRY: OnceLock<ModuleRegistry> = OnceLock::new();

fn get_registry() -> &'static ModuleRegistry {
    REGISTRY.get_or_init(ModuleRegistry::build)
}

impl ModuleRegistry {
    fn build() -> Self {
        let builders = MODULE_CATALOG
            .iter()
            .map(|m| (m.name, (m.builder)()))
            .collect();
        ModuleRegistry { builders }
    }

    pub fn get() -> &'static ModuleRegistry {
        get_registry()
    }

    pub fn run_selected(
        &self,
        selected: &[String],
        ctx: &SharedContext,
        template_content: &str,
    ) -> SystemInfo {
        let mut info = SystemInfo::new();

        let template_modules = extract_template_modules(template_content);

        // Parallel collection (rayon) when the `parallel` feature is on; the
        // minimal build falls back to a plain sequential loop (see ROADMAP 0.2).
        let entry = |name: &String| {
            if name == "separator" {
                return None;
            }
            if !template_modules.is_empty() && !template_modules.contains(name.as_str()) {
                return None;
            }
            crate::find_module(name)
                .map(|m| (m.name, m.builder))
                .map(|(n, builder)| (n, collect_one(n, builder, ctx)))
        };

        #[cfg(feature = "parallel")]
        let entries: Vec<_> = {
            use rayon::prelude::*;
            selected.par_iter().filter_map(entry).collect()
        };
        #[cfg(not(feature = "parallel"))]
        let entries: Vec<_> = selected.iter().filter_map(entry).collect();

        for (name, result) in entries {
            match result {
                Ok(val) => info.add(name, val),
                Err(e) => {
                    if ctx.debug {
                        eprintln!("[flexfetch] module {name} error: {e}");
                    }
                    info.add(name, InfoValue::Scalar("error".into()));
                }
            }
        }

        info
    }

    /// Phase 7.11: snapshot-reuse collection for `--watch` (and other
    /// repeated-run loops). Static modules (OS/host/kernel/… — values that
    /// don't change mid-session) are served from `cache`; only the dynamic
    /// ones (cpuusage/memory/disk/network/battery/… — values that change every
    /// tick) are re-collected. `cache` is updated in place so the next call
    /// reuses this tick's static values. Thread-safe for the parallel path.
    pub fn run_selected_cached(
        &self,
        selected: &[String],
        ctx: &SharedContext,
        template_content: &str,
        cache: &mut std::collections::HashMap<String, InfoValue>,
    ) -> SystemInfo {
        let template_modules = extract_template_modules(template_content);
        let entry = |name: &String| {
            if name == "separator" {
                return None;
            }
            if !template_modules.is_empty() && !template_modules.contains(name.as_str()) {
                return None;
            }
            crate::find_module(name)
                .map(|m| (m.name, m.builder))
                .map(|(n, builder)| (n, collect_one(n, builder, ctx)))
        }; // Derive static/dynamic from catalog — no hardcoded list to keep in sync.
        let is_static = |name: &str| MODULE_CATALOG.iter().any(|m| m.name == name && m.is_static);

        // Reuse cached static values that are still requested; collect the rest.
        let mut info = SystemInfo::new();
        let mut to_collect: Vec<String> = Vec::new();
        for name in selected {
            if name == "separator" {
                continue;
            }
            if !template_modules.is_empty() && !template_modules.contains(name.as_str()) {
                continue;
            }
            if is_static(name) {
                if let Some(cached) = cache.get(name) {
                    // Resolve the canonical &'static name from the builder list
                    // (module names are static literals) — avoids leaking a
                    // String per cache hit on every watch tick.
                    if let Some(static_name) = self
                        .builders
                        .iter()
                        .find(|(n, _)| *n == name)
                        .map(|(n, _)| *n)
                    {
                        info.add(static_name, cached.clone());
                        continue;
                    }
                }
            }
            to_collect.push(name.clone());
        }

        #[cfg(feature = "parallel")]
        let entries: Vec<_> = {
            use rayon::prelude::*;
            to_collect.par_iter().filter_map(entry).collect()
        };
        #[cfg(not(feature = "parallel"))]
        let entries: Vec<_> = to_collect.iter().filter_map(entry).collect();

        for (name, result) in entries {
            match result {
                Ok(val) => {
                    // Cache static modules so the next tick reuses them.
                    if is_static(name) {
                        cache.insert(name.to_string(), val.clone());
                    }
                    info.add(name, val);
                }
                Err(e) => {
                    if ctx.debug {
                        eprintln!("[flexfetch] module {name} error: {e}");
                    }
                    info.add(name, InfoValue::Scalar("error".into()));
                }
            }
        }

        info
    }

    pub fn run_individual(&self, name: &str, ctx: &Context) -> Option<InfoValue> {
        self.builders
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, module)| {
                module
                    .collect(ctx)
                    .unwrap_or(InfoValue::Scalar("error".into()))
            })
    }
}
