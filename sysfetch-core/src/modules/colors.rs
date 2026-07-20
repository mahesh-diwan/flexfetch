use crate::{Module, InfoValue, Context, Result};

pub struct ColorsModule;

impl Module for ColorsModule {
    fn name(&self) -> &'static str { "colors" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let swatches = (0..8).map(|i| color_swatch(i)).collect();
        Ok(InfoValue::List(swatches))
    }
}

fn color_swatch(n: u8) -> String {
    format!("\x1b[48;5;{code}m  \x1b[0m", code = n)
}
