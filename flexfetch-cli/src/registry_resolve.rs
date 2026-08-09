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
