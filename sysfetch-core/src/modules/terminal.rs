use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct TerminalModule;

impl Module for TerminalModule {
    fn name(&self) -> &'static str { "terminal" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let program = std::env::var("TERM_PROGRAM")
            .or_else(|_| std::env::var("TERM"))
            .unwrap_or_else(|_| "unknown".into());
        let version = std::env::var("TERM_PROGRAM_VERSION").ok();

        if let Some(v) = version {
            let mut map = HashMap::new();
            map.insert("name".into(), program);
            map.insert("version".into(), v);
            Ok(InfoValue::Map(map))
        } else {
            Ok(InfoValue::Scalar(program))
        }
    }
}
