use crate::{Context, InfoValue, Module, SystemInfo, MODULE_CATALOG};
use std::collections::HashSet;
use std::sync::OnceLock;

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
        ctx: &Context,
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
            self.builders
                .iter()
                .find(|(n, _)| n == name)
                .map(|(n, module)| {
                    let result = module.collect(ctx);
                    (*n, result)
                })
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
        ctx: &Context,
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
            self.builders
                .iter()
                .find(|(n, _)| n == name)
                .map(|(n, module)| {
                    let result = module.collect(ctx);
                    (*n, result)
                })
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
