use crate::ansi::{ANSI_BRIGHT_COLORS, ANSI_COLORS};
use crate::{Context, InfoValue, Module, Result};

pub struct ColorsModule;

impl Module for ColorsModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        // Return RGB values as "r,g,b" strings for palette_display filter
        let rgb_colors: Vec<String> = ANSI_COLORS
            .iter()
            .chain(ANSI_BRIGHT_COLORS.iter())
            .map(|c| format!("{},{},{}", c[0], c[1], c[2]))
            .collect();
        Ok(InfoValue::List(rgb_colors))
    }
}
