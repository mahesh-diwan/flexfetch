mod ansi;
#[cfg(feature = "auto-theme")]
pub mod autotheme;
pub mod cache;
pub mod config;
pub mod context;
pub mod error;
pub mod export;
pub mod fastfetch_logos;
pub mod hardware_db;
pub mod image_logo;
pub mod logo;
pub mod logo_data;
pub mod module;
pub mod module_registry;
pub mod template;
pub mod theme;
// Phase 8.9 — Windows FFI helpers (registry reads, UTF-16 conversion). Only
// compiled on Windows targets; Linux/macOS never see the module.
#[cfg(target_os = "windows")]
pub mod win;

pub mod modules;
pub mod presets;

pub use cache::get_cache_dir;
pub use config::Config;
pub use context::Context;
pub use error::{Error, Result};
pub use image_logo::{
    get_distro_logo_path, get_module_logo_path, ImageLogo, ImageProtocol, LogoMode,
};
pub use module::{find_module, InfoValue, Module, ModuleEntry, SystemInfo, MODULE_CATALOG};
pub use module_registry::ModuleRegistry;
pub use template::TeraEngine;

#[cfg(test)]
mod tests {
    #[test]
    fn test_uptime_format() {
        assert_eq!(crate::modules::uptime::format_uptime(3661), "1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(90061), "1d 1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(7200), "2h 0m");
    }
}
