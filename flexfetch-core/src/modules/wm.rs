use crate::{Context, InfoValue, Module, Result};

pub struct WmModule;

impl Module for WmModule {
    fn name(&self) -> &'static str {
        "wm"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
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

        let value = if wm == "unknown" {
            de.map(|d| format!("{} (DE)", d))
                .unwrap_or_else(|| "unknown".to_string())
        } else {
            wm
        };

        Ok(InfoValue::Scalar(value))
    }
}
