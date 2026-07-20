use crate::{Context, InfoValue, Module, Result};

pub struct ColorsModule;

impl Module for ColorsModule {
    fn name(&self) -> &'static str {
        "colors"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut blocks = Vec::new();
        for i in 0..16 {
            let code = if i < 8 { 30 + i } else { 82 + i };
            blocks.push(format!("\x1b[{}m\u{2588}\u{2588}\x1b[0m", code));
        }
        Ok(InfoValue::List(blocks))
    }
}
