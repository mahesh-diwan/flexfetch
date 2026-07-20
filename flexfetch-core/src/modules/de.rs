use crate::{Context, InfoValue, Module, Result};

pub struct DeModule;

impl Module for DeModule {
    fn name(&self) -> &'static str {
        "de"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let de = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .or_else(|_| std::env::var("GDMSESSION"))
            .unwrap_or_else(|_| "unknown".to_string());

        Ok(InfoValue::Scalar(de))
    }
}
