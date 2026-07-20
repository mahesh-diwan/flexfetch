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
        // Stub modules (battery, colors, cpu, custom, de, disk, gpu, memory,
        // network, packages, processes, shell, terminal, wm) added when
        // Tasks 4/5 complete them.
        builders.push(("os", || Box::new(crate::modules::os::OsModule)));
        builders.push(("host", || Box::new(crate::modules::host::HostModule)));
        builders.push(("kernel", || Box::new(crate::modules::kernel::KernelModule)));
        builders.push(("uptime", || Box::new(crate::modules::uptime::UptimeModule)));
        builders.push(("locale", || Box::new(crate::modules::locale::LocaleModule)));
        builders.push(("colors", || Box::new(crate::modules::colors::ColorsModule)));

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
