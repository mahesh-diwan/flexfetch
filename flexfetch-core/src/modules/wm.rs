use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct WmModule;

impl Module for WmModule {
    fn name(&self) -> &'static str {
        "wm"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        let de = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .or_else(|_| std::env::var("GDMSESSION"))
            .ok();

        let wm = std::process::Command::new("gsettings")
            .args(["get", "org.gnome.desktop.wm.preferences", "current"])
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
            .map(|s| s.trim_matches('\'').to_string())
            .or_else(|| de.clone())
            .unwrap_or_else(|| "unknown".to_string());

        let name = if wm == "unknown" {
            de.unwrap_or_else(|| "unknown".to_string())
        } else {
            wm
        };
        map.insert("name".into(), name);

        // Theme / icons / cursor
        for (schema, key, field) in [
            ("org.gnome.desktop.interface", "gtk-theme", "theme"),
            ("org.gnome.desktop.interface", "icon-theme", "icons"),
            ("org.gnome.desktop.interface", "cursor-theme", "cursor"),
            ("org.gnome.desktop.interface", "font-name", "font"),
        ] {
            if let Ok(output) = std::process::Command::new("gsettings")
                .args(["get", schema, key])
                .output()
            {
                let v = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .trim_matches('\'')
                    .to_string();
                if !v.is_empty() && v != "default" {
                    map.insert(field.into(), v);
                }
            }
        }

        Ok(InfoValue::Map(map))
    }
}
