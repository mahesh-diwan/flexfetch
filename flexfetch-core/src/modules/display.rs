use crate::{Context, InfoValue, Module, Result};
use std::process::Command;

pub struct DisplayModule;

impl Module for DisplayModule {
    fn name(&self) -> &'static str {
        "display"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        // Try wlr-randr first (Wayland-native)
        if let Some(v) = try_wlr_randr() {
            return Ok(InfoValue::Scalar(v));
        }
        // Try xrandr
        if let Some(v) = try_xrandr() {
            return Ok(InfoValue::Scalar(v));
        }
        // Fallback: DRM sysfs
        if let Some(v) = try_drm() {
            return Ok(InfoValue::Scalar(v));
        }
        Ok(InfoValue::Scalar("unknown".into()))
    }
}

fn try_wlr_randr() -> Option<String> {
    let out = Command::new("wlr-randr").output().ok()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    parse_wlr_randr(&stdout)
}

fn parse_wlr_randr(output: &str) -> Option<String> {
    for line in output.lines() {
        // Lines like: "  1920x1080 px, 60.00 Hz (preferred, current)"
        let trimmed = line.trim();
        if !trimmed.contains("current") {
            continue;
        }
        let res = trimmed.split_whitespace().next()?;
        let hz = trimmed
            .split_whitespace()
            .find(|s| s.contains("Hz"))?
            .trim_end_matches("Hz")
            .trim_end_matches(" (preferred")
            .trim();
        return Some(format!("{res} @ {hz}"));
    }
    None
}

fn try_xrandr() -> Option<String> {
    let out = Command::new("xrandr").arg("--query").output().ok()?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in stdout.lines() {
        // Lines like: "1920x1080     60.00*+  50.00    59.94"
        if !line.contains('*') {
            continue;
        }
        let res = line.split_whitespace().next()?;
        let hz = line
            .split_whitespace()
            .skip(1)
            .find(|s| s.contains('*'))?
            .trim_end_matches('*')
            .trim_end_matches('+');
        return Some(format!("{res} @ {hz}Hz"));
    }
    None
}

fn try_drm() -> Option<String> {
    let entries = std::fs::read_dir("/sys/class/drm").ok()?;
    for entry in entries.flatten() {
        let modes_path = entry.path().join("modes");
        if modes_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&modes_path) {
                if let Some(mode) = content.lines().next() {
                    let mode = mode.trim();
                    if !mode.is_empty() {
                        return Some(mode.to_string());
                    }
                }
            }
        }
    }
    None
}
