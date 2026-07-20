use crate::config::Config;
use crate::{Context, InfoValue, Module, SystemInfo};

type ModuleBuilder = fn() -> Box<dyn Module>;

pub struct ModuleRegistry {
    builders: Vec<(&'static str, ModuleBuilder)>,
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
        builders.push(("custom", || {
            Box::new(crate::modules::custom::CustomCommandsModule)
        }));

        ModuleRegistry { builders }
    }

    pub fn run_selected(&self, selected: &[String], ctx: &Context) -> SystemInfo {
        use rayon::prelude::*;
        let mut info = SystemInfo::new();

        let entries: Vec<_> = selected
            .par_iter()
            .filter_map(|name| {
                if name == "title" || name == "separator" {
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
}
