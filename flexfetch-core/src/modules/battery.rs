use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct BatteryModule;

impl Module for BatteryModule {
    fn name(&self) -> &'static str {
        "battery"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name = name.to_str().unwrap_or("");
                    if !name.starts_with("BAT") {
                        continue;
                    }
                    let base = entry.path();
                    if let Ok(cap) = std::fs::read_to_string(base.join("capacity")) {
                        map.insert("percent".into(), format!("{}%", cap.trim()));
                    }
                    if let Ok(status) = std::fs::read_to_string(base.join("status")) {
                        map.insert("status".into(), status.trim().to_string());
                    }
                    break;
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("pmset")
                .args(["-g", "batt"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.contains('%') {
                        if let Some(pct) = line.split('\t').nth(1) {
                            if let Some(charge) = pct.split(';').nth(0) {
                                map.insert("percent".into(), charge.trim().to_string());
                            }
                            if let Some(state) = pct.split(';').nth(1) {
                                map.insert("status".into(), state.trim().to_string());
                            }
                            if let Some(remaining) = pct.split(';').nth(2) {
                                let t = remaining.trim();
                                if !t.is_empty() {
                                    map.insert("time".into(), t.to_string());
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }

        if map.is_empty() {
            return Ok(InfoValue::Scalar("unknown".into()));
        }
        Ok(InfoValue::Map(map))
    }
}
