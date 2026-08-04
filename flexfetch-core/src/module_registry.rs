use crate::{Context, InfoValue, Module, SystemInfo};
use std::collections::HashSet;
use std::sync::OnceLock;

type ModuleBuilder = Box<dyn Module>;

fn extract_template_modules(template_str: &str) -> HashSet<String> {
    let mut modules = HashSet::new();
    let known = [
        "os",
        "host",
        "kernel",
        "uptime",
        "packages",
        "shell",
        "terminal",
        "de",
        "wm",
        "cpu",
        "cpucache",
        "cpuusage",
        "memory",
        "gpu",
        "disk",
        "display",
        "network",
        "battery",
        "locale",
        "resolution",
        "colors",
        "custom",
        "processes",
        "title",
        "bluetooth",
        "media",
        "dns",
        "temperature",
        "swap",
        "publicip",
        "wifi",
        "git",
        "project",
        "context",
        "health",
        "weather",
        "container",
        "wallpaper",
        "fsdeep",
    ];
    for word in known {
        if template_str.contains(word) {
            modules.insert(word.to_string());
        }
    }
    modules
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
        let builders = vec![
            (
                "os",
                Box::new(crate::modules::os::OsModule) as Box<dyn Module>,
            ),
            (
                "host",
                Box::new(crate::modules::host::HostModule) as Box<dyn Module>,
            ),
            (
                "kernel",
                Box::new(crate::modules::kernel::KernelModule) as Box<dyn Module>,
            ),
            (
                "uptime",
                Box::new(crate::modules::uptime::UptimeModule) as Box<dyn Module>,
            ),
            (
                "locale",
                Box::new(crate::modules::locale::LocaleModule) as Box<dyn Module>,
            ),
            (
                "colors",
                Box::new(crate::modules::colors::ColorsModule) as Box<dyn Module>,
            ),
            (
                "de",
                Box::new(crate::modules::de::DeModule) as Box<dyn Module>,
            ),
            (
                "packages",
                Box::new(crate::modules::packages::PackagesModule) as Box<dyn Module>,
            ),
            (
                "shell",
                Box::new(crate::modules::shell::ShellModule) as Box<dyn Module>,
            ),
            (
                "terminal",
                Box::new(crate::modules::terminal::TerminalModule) as Box<dyn Module>,
            ),
            (
                "wm",
                Box::new(crate::modules::wm::WmModule) as Box<dyn Module>,
            ),
            (
                "cpu",
                Box::new(crate::modules::cpu::CpuModule) as Box<dyn Module>,
            ),
            (
                "memory",
                Box::new(crate::modules::memory::MemoryModule) as Box<dyn Module>,
            ),
            (
                "processes",
                Box::new(crate::modules::processes::ProcessesModule) as Box<dyn Module>,
            ),
            (
                "battery",
                Box::new(crate::modules::battery::BatteryModule) as Box<dyn Module>,
            ),
            (
                "gpu",
                Box::new(crate::modules::gpu::GpuModule) as Box<dyn Module>,
            ),
            (
                "disk",
                Box::new(crate::modules::disk::DiskModule) as Box<dyn Module>,
            ),
            (
                "network",
                Box::new(crate::modules::network::NetworkModule) as Box<dyn Module>,
            ),
            (
                "resolution",
                Box::new(crate::modules::resolution::ResolutionModule) as Box<dyn Module>,
            ),
            (
                "title",
                Box::new(crate::modules::title::TitleModule) as Box<dyn Module>,
            ),
            (
                "custom",
                Box::new(crate::modules::custom::CustomCommandsModule) as Box<dyn Module>,
            ),
            (
                "bluetooth",
                Box::new(crate::modules::bluetooth::BluetoothModule) as Box<dyn Module>,
            ),
            (
                "media",
                Box::new(crate::modules::media::MediaModule) as Box<dyn Module>,
            ),
            (
                "temperature",
                Box::new(crate::modules::temperature::TemperatureModule) as Box<dyn Module>,
            ),
            (
                "dns",
                Box::new(crate::modules::dns::DnsModule) as Box<dyn Module>,
            ),
            (
                "swap",
                Box::new(crate::modules::swap::SwapModule) as Box<dyn Module>,
            ),
            (
                "cpucache",
                Box::new(crate::modules::cpucache::CpuCacheModule) as Box<dyn Module>,
            ),
            (
                "cpuusage",
                Box::new(crate::modules::cpuusage::CpuUsageModule) as Box<dyn Module>,
            ),
            (
                "display",
                Box::new(crate::modules::display::DisplayModule) as Box<dyn Module>,
            ),
            (
                "publicip",
                Box::new(crate::modules::publicip::PublicIpModule) as Box<dyn Module>,
            ),
            (
                "wifi",
                Box::new(crate::modules::wifi::WifiModule) as Box<dyn Module>,
            ),
            (
                "git",
                Box::new(crate::modules::git::GitModule) as Box<dyn Module>,
            ),
            (
                "project",
                Box::new(crate::modules::project::ProjectModule) as Box<dyn Module>,
            ),
            (
                "context",
                Box::new(crate::modules::context::ContextModule) as Box<dyn Module>,
            ),
            (
                "health",
                Box::new(crate::modules::health::HealthModule) as Box<dyn Module>,
            ),
            (
                "weather",
                Box::new(crate::modules::weather::WeatherModule) as Box<dyn Module>,
            ),
            (
                "container",
                Box::new(crate::modules::container::ContainerModule) as Box<dyn Module>,
            ),
            (
                "wallpaper",
                Box::new(crate::modules::wallpaper::WallpaperModule) as Box<dyn Module>,
            ),
            (
                "fsdeep",
                Box::new(crate::modules::fsdeep::FsDeepModule) as Box<dyn Module>,
            ),
        ];

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
        };

        // Modules whose values are effectively static within a session.
        let static_modules: [&str; 23] = [
            "os",
            "host",
            "kernel",
            "shell",
            "terminal",
            "de",
            "wm",
            "locale",
            "packages",
            "cpu",
            "cpucache",
            "gpu",
            "colors",
            "resolution",
            "display",
            "project",
            "git",
            "context",
            "container",
            "wallpaper",
            "fsdeep",
            "weather",
            "custom",
        ];
        let is_static = |name: &str| static_modules.contains(&name);

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
