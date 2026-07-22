use crate::{Context, InfoValue, Module, Result};

pub struct TitleModule;

impl Module for TitleModule {
    fn name(&self) -> &'static str {
        "title"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let user = std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .unwrap_or_else(|_| "user".to_string());
        let hostname = std::fs::read_to_string("/etc/hostname")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| {
                std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())
            });
        Ok(InfoValue::Scalar(format!("{}@{}", user, hostname)))
    }
}
