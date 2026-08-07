use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct WmModule;

impl Module for WmModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        let de = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .or_else(|_| std::env::var("GDMSESSION"))
            .ok();

        // WM name: env vars first (zero-spawn), then gsettings `current-wm`
        // (GNOME keeps the real WM — e.g. mutter — only in dconf, so env alone
        // would report "gnome" instead of the actual window manager).
        let wm = env_wm_name()
            .or_else(|| gsettings_value("org.gnome.desktop.wm.preferences", "current-wm"))
            .or_else(|| de.clone())
            .unwrap_or_else(|| "unknown".to_string());
        map.insert("name".into(), wm);

        // Phase 4.1: read GTK theme/icons/cursor/font from ~/.config/gtk-*.ini
        // instead of spawning gsettings per key (zero subprocesses on the
        // default path). Falls back to gsettings only when no config files exist.
        let gtk = gtk_config();
        let mut from_config = false;
        for (key, field) in [
            ("gtk-theme-name", "theme"),
            ("gtk-icon-theme-name", "icons"),
            ("gtk-cursor-theme-name", "cursor"),
            ("gtk-font-name", "font"),
        ] {
            if let Some(v) = gtk.get(key) {
                map.insert(field.into(), v.clone());
                from_config = true;
            }
        }

        if !from_config {
            for (schema, key, field) in [
                ("org.gnome.desktop.interface", "gtk-theme", "theme"),
                ("org.gnome.desktop.interface", "icon-theme", "icons"),
                ("org.gnome.desktop.interface", "cursor-theme", "cursor"),
                ("org.gnome.desktop.interface", "font-name", "font"),
            ] {
                if let Some(v) = gsettings_value(schema, key) {
                    map.insert(field.into(), v);
                }
            }
        }

        Ok(InfoValue::Map(map))
    }
}

/// WM name from the session env vars (no xprop/xrandr spawn).
fn env_wm_name() -> Option<String> {
    std::env::var("XDG_SESSION_DESKTOP")
        .or_else(|_| std::env::var("XDG_CURRENT_DESKTOP"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// One `gsettings get` call, returning the bare value (no quotes).
fn gsettings_value(schema: &str, key: &str) -> Option<String> {
    let output = std::process::Command::new("gsettings")
        .args(["get", schema, key])
        .output()
        .ok()?;
    let v = String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_matches('\'')
        .to_string();
    if v.is_empty() || v == "default" {
        None
    } else {
        Some(v)
    }
}

/// Parse every `gtk-*` key from ~/.config/gtk-3.0/settings.ini and
/// gtk-4.0/settings.ini into one map (each file read at most once).
fn gtk_config() -> HashMap<String, String> {
    let mut out = HashMap::new();
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return out,
    };
    for rel in ["gtk-3.0/settings.ini", "gtk-4.0/settings.ini"] {
        let path = format!("{home}/.config/{rel}");
        if let Ok(content) = std::fs::read_to_string(&path) {
            for line in content.lines() {
                let line = line.trim();
                if let Some((k, v)) = line.split_once('=') {
                    let v = v.trim().trim_matches('"').trim_matches('\'');
                    if !v.is_empty() && v != "default" {
                        out.insert(k.trim().to_string(), v.to_string());
                    }
                }
            }
        }
    }
    out
}
