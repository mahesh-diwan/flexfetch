use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct WallpaperModule;

impl Module for WallpaperModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        // Phase 4.8: wallpaper path + desktop context from per-DE config files
        // (no spawns on the common paths — gsettings only as a GNOME fallback).
        let mut map = HashMap::new();

        if let Some(path) = detect_wallpaper() {
            map.insert("path".into(), path);
            if let Some(name) = wallpaper_name(&map["path"]) {
                map.insert("file".into(), name);
            }
        }

        // GTK theme context (theme/icons/cursor) from the wm module's source.
        if let Some(t) = gtk_theme("gtk-theme-name") {
            map.insert("gtk_theme".into(), t);
        }
        if let Some(t) = gtk_theme("gtk-icon-theme-name") {
            map.insert("icon_theme".into(), t);
        }
        if let Some(t) = gtk_theme("gtk-cursor-theme-name") {
            map.insert("cursor_theme".into(), t);
        }

        Ok(InfoValue::Map(map))
    }
}

/// Detect the current wallpaper path per DE/WM (sway, hyprland, KDE, feh,
/// nitrogen, variety, GNOME gsettings fallback). No spawns on the common paths.
/// `pub(crate)`: reused by the Phase 5.4 `auto-theme` module.
pub(crate) fn detect_wallpaper() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let cfg = PathBuf::from(&home).join(".config");

    // Sway: `output * bg /path/to/img ...`
    if let Ok(content) = std::fs::read_to_string(cfg.join("sway/config")) {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("output") && line.contains("bg ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(idx) = parts.iter().position(|p| *p == "bg") {
                    if let Some(p) = parts.get(idx + 1) {
                        if p.starts_with('/') {
                            return Some(p.to_string());
                        }
                    }
                }
            }
        }
    }

    // Hyprland: `wallpaper = ,/path/to/img`
    for conf in ["hypr/hyprland.conf", "hypr/hyprpaper.conf"] {
        if let Ok(content) = std::fs::read_to_string(cfg.join(conf)) {
            for line in content.lines() {
                if let Some(rest) = line.trim().strip_prefix("wallpaper") {
                    if let Some(eq) = rest.find('=') {
                        let v = rest[eq + 1..].trim();
                        let path = v.rsplit(',').next().unwrap_or("").trim();
                        if path.starts_with('/') && !path.is_empty() {
                            return Some(path.to_string());
                        }
                    }
                }
            }
        }
    }

    // KDE Plasma: parse plasma-org.kde.plasma.desktop-appletsrc for Image=
    if let Ok(content) =
        std::fs::read_to_string(cfg.join("plasma-org.kde.plasma.desktop-appletsrc"))
    {
        for line in content.lines() {
            if let Some(v) = line.trim().strip_prefix("Image=") {
                let v = v.trim().trim_matches('"');
                if !v.is_empty() {
                    return Some(v.to_string());
                }
            }
        }
    }

    // feh / nitrogen configs.
    for f in [
        ".fehbg",
        ".config/nitrogen/bg-saved.cfg",
        ".config/variety/variety.conf",
    ] {
        let path = PathBuf::from(&home).join(f);
        if let Ok(content) = std::fs::read_to_string(&path) {
            for line in content.lines() {
                let line = line.trim();
                if line.contains("feh") && line.contains("--bg") {
                    if let Some(path) = line.split_whitespace().rev().find(|p| p.starts_with('/')) {
                        return Some(path.to_string());
                    }
                }
                if let Some(rest) = line.strip_prefix("file=") {
                    let v = rest.trim().trim_matches('"');
                    if !v.is_empty() {
                        return Some(v.to_string());
                    }
                }
            }
        }
    }

    // GNOME: gsettings (spawn — only reached when no config file matched).
    if let Ok(out) = std::process::Command::new("gsettings")
        .args(["get", "org.gnome.desktop.background", "picture-uri"])
        .output()
    {
        let v = String::from_utf8_lossy(&out.stdout)
            .trim()
            .trim_matches('\'')
            .to_string();
        if let Some(path) = v.strip_prefix("file://") {
            return Some(path.to_string());
        }
    }

    None
}

fn wallpaper_name(path: &str) -> Option<String> {
    PathBuf::from(path)
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
}

/// Read a gtk-* key from the gtk settings.ini files (no spawn).
fn gtk_theme(key: &str) -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    for rel in ["gtk-3.0/settings.ini", "gtk-4.0/settings.ini"] {
        let path = format!("{home}/.config/{rel}");
        if let Ok(content) = std::fs::read_to_string(&path) {
            for line in content.lines() {
                let line = line.trim();
                if let Some((k, v)) = line.split_once('=') {
                    if k.trim() == key {
                        let v = v.trim().trim_matches('"').trim_matches('\'');
                        if !v.is_empty() && v != "default" {
                            return Some(v.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn parses_sway_bg_line() {
        let line = "output * bg /usr/share/wallpapers/mountain.jpg fill";
        let parts: Vec<&str> = line.split_whitespace().collect();
        let idx = parts.iter().position(|p| *p == "bg").unwrap();
        assert_eq!(parts[idx + 1], "/usr/share/wallpapers/mountain.jpg");
    }

    #[test]
    fn parses_hyprland_wallpaper() {
        let line = "wallpaper = ,/home/u/Pictures/wall.png";
        let rest = line.trim().strip_prefix("wallpaper").unwrap();
        let eq = rest.find('=').unwrap();
        let v = rest[eq + 1..].trim();
        let path = v.rsplit(',').next().unwrap().trim();
        assert_eq!(path, "/home/u/Pictures/wall.png");
    }
}
