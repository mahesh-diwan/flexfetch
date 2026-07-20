use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct TerminalModule;

impl Module for TerminalModule {
    fn name(&self) -> &'static str {
        "terminal"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        let name = std::env::var("TERM_PROGRAM")
            .or_else(|_| std::env::var("TERM"))
            .unwrap_or_else(|_| "unknown".to_string());
        map.insert("name".into(), name);

        // Font detection: try kitty, fallback env var
        let font = std::process::Command::new("kitty")
            .args(["@", "get-font"])
            .output()
            .ok()
            .and_then(|o| {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            })
            .or_else(|| {
                std::env::var("TERMINAL_FONT")
                    .ok()
                    .filter(|s| !s.is_empty())
            });

        if let Some(f) = font {
            map.insert("font".into(), f);
        }

        Ok(InfoValue::Map(map))
    }
}
