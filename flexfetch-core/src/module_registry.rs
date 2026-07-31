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
        "memory",
        "gpu",
        "disk",
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
        use rayon::prelude::*;
        let mut info = SystemInfo::new();

        let template_modules = extract_template_modules(template_content);

        let entries: Vec<_> = selected
            .par_iter()
            .filter_map(|name| {
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
            })
            .collect();

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
