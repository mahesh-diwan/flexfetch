use crate::{Module, InfoValue, Context, Result};

pub struct DeModule;

impl Module for DeModule {
    fn name(&self) -> &'static str { "de" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let de = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
            .unwrap_or_else(|_| "unknown".into());
        Ok(InfoValue::Scalar(de))
    }
}
