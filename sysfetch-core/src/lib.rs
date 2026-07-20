pub mod module;
pub mod context;
pub mod config;
pub mod template;
pub mod error;

pub mod modules;

pub use module::{Module, InfoValue, SystemInfo};
pub use context::Context;
pub use config::Config;
pub use template::TeraEngine;
pub use error::{Error, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptime_format() {
        assert_eq!(crate::modules::uptime::format_uptime(3661), "1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(90061), "1d 1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(120), "2h 0m");
    }
}
