use crate::{Context, InfoValue, Module, Result};

pub struct ColorsModule;

impl Module for ColorsModule {
    fn name(&self) -> &'static str {
        "colors"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        // Return RGB values as "r,g,b" strings for palette_display filter
        let rgb_colors: Vec<String> = vec![
            "0,0,0".into(),       // 30 black
            "170,0,0".into(),     // 31 red
            "0,170,0".into(),     // 32 green
            "170,85,0".into(),    // 33 yellow
            "0,0,170".into(),     // 34 blue
            "170,0,170".into(),   // 35 magenta
            "0,170,170".into(),   // 36 cyan
            "170,170,170".into(), // 37 white
            "85,85,85".into(),    // 90 bright black
            "255,85,85".into(),   // 91 bright red
            "85,255,85".into(),   // 92 bright green
            "255,255,85".into(),  // 93 bright yellow
            "85,85,255".into(),   // 94 bright blue
            "255,85,255".into(),  // 95 bright magenta
            "85,255,255".into(),  // 96 bright cyan
            "255,255,255".into(), // 97 bright white
        ];
        Ok(InfoValue::List(rgb_colors))
    }
}
