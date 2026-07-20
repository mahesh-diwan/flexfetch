use crate::{Context, InfoValue, Module, Result};

pub struct TerminalModule;

impl Module for TerminalModule {
    fn name(&self) -> &'static str {
        "terminal"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let terminal = std::env::var("TERM_PROGRAM")
            .or_else(|_| std::env::var("TERM"))
            .unwrap_or_else(|_| "unknown".to_string());

        Ok(InfoValue::Scalar(terminal))
    }
}
