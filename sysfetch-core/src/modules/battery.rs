use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct BatteryModule;

impl Module for BatteryModule {
    fn name(&self) -> &'static str { "battery" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let type_path = path.join("type");
                    let typ = std::fs::read_to_string(&type_path).unwrap_or_default();
                    if typ.trim() != "Battery" { continue; }
                    let capacity = std::fs::read_to_string(path.join("capacity")).ok()
                        .and_then(|s| s.trim().parse::<u8>().ok()).unwrap_or(0);
                    let status = std::fs::read_to_string(path.join("status"))
                        .unwrap_or_default().trim().to_string();
                    map.insert("percent".into(), format!("{capacity}%"));
                    map.insert("status".into(), status);
                    break;
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(out) = std::process::Command::new("pmset")
                .args(["-g", "batt"])
                .output()
            {
                let s = String::from_utf8_lossy(&out.stdout);
                for line in s.lines() {
                    if let Some(info) = line.trim().strip_prefix("-InternalBattery-0") {
                        if let Some(pct) = info.split_whitespace()
                            .find(|w| w.ends_with('%'))
                        {
                            map.insert("percent".into(), pct.to_string());
                        }
                        if info.contains("discharging") {
                            map.insert("status".into(), "Discharging".into());
                        } else if info.contains("charging") || info.contains("AC") {
                            map.insert("status".into(), "Charging".into());
                        } else if info.contains("charged") {
                            map.insert("status".into(), "Full".into());
                        }
                        break;
                    }
                }
            }
        }

        if map.is_empty() {
            return Ok(InfoValue::Scalar("no battery".into()));
        }
        Ok(InfoValue::Map(map))
    }
}
