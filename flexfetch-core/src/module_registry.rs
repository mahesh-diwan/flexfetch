use crate::config::Config;
use crate::{Context, InfoValue, Module, SystemInfo};
use std::collections::HashSet;

type ModuleBuilder = fn() -> Box<dyn Module>;

pub struct ModuleRegistry {
    builders: Vec<(&'static str, ModuleBuilder)>,
}

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
    ];
    for word in known {
        if template_str.contains(word) {
            modules.insert(word.to_string());
        }
    }
    modules
}

impl ModuleRegistry {
    pub fn new(_config: &Config) -> Self {
        let mut builders: Vec<(&'static str, ModuleBuilder)> = Vec::new();

        // Only modules with implemented structs are registered.
        builders.push(("os", || Box::new(crate::modules::os::OsModule)));
        builders.push(("host", || Box::new(crate::modules::host::HostModule)));
        builders.push(("kernel", || Box::new(crate::modules::kernel::KernelModule)));
        builders.push(("uptime", || Box::new(crate::modules::uptime::UptimeModule)));
        builders.push(("locale", || Box::new(crate::modules::locale::LocaleModule)));
        builders.push(("colors", || Box::new(crate::modules::colors::ColorsModule)));
        builders.push(("de", || Box::new(crate::modules::de::DeModule)));
        builders.push(("packages", || {
            Box::new(crate::modules::packages::PackagesModule)
        }));
        builders.push(("shell", || Box::new(crate::modules::shell::ShellModule)));
        builders.push(("terminal", || {
            Box::new(crate::modules::terminal::TerminalModule)
        }));
        builders.push(("wm", || Box::new(crate::modules::wm::WmModule)));
        builders.push(("cpu", || Box::new(crate::modules::cpu::CpuModule)));
        builders.push(("memory", || Box::new(crate::modules::memory::MemoryModule)));
        builders.push(("processes", || {
            Box::new(crate::modules::processes::ProcessesModule)
        }));
        builders.push(("battery", || {
            Box::new(crate::modules::battery::BatteryModule)
        }));
        builders.push(("gpu", || Box::new(crate::modules::gpu::GpuModule)));
        builders.push(("disk", || Box::new(crate::modules::disk::DiskModule)));
        builders.push(("network", || {
            Box::new(crate::modules::network::NetworkModule)
        }));
        builders.push(("resolution", || {
            Box::new(crate::modules::resolution::ResolutionModule)
        }));
        builders.push(("title", || Box::new(crate::modules::title::TitleModule)));
        builders.push(("custom", || {
            Box::new(crate::modules::custom::CustomCommandsModule)
        }));

        ModuleRegistry { builders }
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
                    .map(|(n, builder)| {
                        let module = builder();
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
            .map(|(_, builder)| {
                let module = builder();
                module
                    .collect(ctx)
                    .unwrap_or(InfoValue::Scalar("error".into()))
            })
    }
}
