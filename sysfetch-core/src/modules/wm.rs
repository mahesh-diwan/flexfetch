use crate::{Module, InfoValue, Context, Result};

pub struct WmModule;

impl Module for WmModule {
    fn name(&self) -> &'static str { "wm" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let wm = detect_wm();
        Ok(InfoValue::Scalar(wm))
    }
}

fn detect_wm() -> String {
    if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
        let de_lower = de.to_lowercase();
        if !de_lower.contains("gnome") && !de_lower.contains("kde")
            && !de_lower.contains("xfce") && !de_lower.contains("cinnamon")
            && !de_lower.contains("mate") && !de_lower.contains("budgie")
            && !de_lower.contains("pantheon")
        {
            return de;
        }
    }

    let known_wms = ["i3", "sway", "hyprland", "bspwm", "dwm", "awesome",
                      "qtile", "openbox", "fluxbox", "xfwm4", "marco",
                      "muffin", "kwin_x11", "kwin_wayland"];

    for wm in known_wms {
        if process_exists(wm) {
            return wm.to_string();
        }
    }

    "unknown".into()
}

fn process_exists(name: &str) -> bool {
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = std::fs::read_dir("/proc") {
            for entry in entries.flatten() {
                let path = entry.path().join("comm");
                if let Ok(comm) = std::fs::read_to_string(&path) {
                    if comm.trim() == name {
                        return true;
                    }
                }
            }
        }
        false
    }
    #[cfg(target_os = "macos")]
    {
        let out = std::process::Command::new("pgrep")
            .args(["-x", name]).output();
        matches!(out, Ok(o) if o.status.success())
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    { false }
}
