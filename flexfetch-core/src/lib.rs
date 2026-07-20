pub mod cache;
pub mod config;
pub mod context;
pub mod error;
pub mod module;
pub mod module_registry;
pub mod template;

pub mod modules;

pub use config::Config;
pub use context::Context;
pub use error::{Error, Result};
pub use module::{InfoValue, Module, SystemInfo};
pub use module_registry::ModuleRegistry;
pub use template::TeraEngine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptime_format() {
        assert_eq!(crate::modules::uptime::format_uptime(3661), "1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(90061), "1d 1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(7200), "2h 0m");
    }
}
