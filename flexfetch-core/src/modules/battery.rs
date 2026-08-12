#![cfg_attr(not(unix), allow(unused_mut))] // collectors mutate only on unix

use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct BatteryModule;

impl Module for BatteryModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = ctx.read_dir("/sys/class/power_supply") {
                for base in entries {
                    let name = base
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    if !name.starts_with("BAT") {
                        continue;
                    }
                    if let Ok(cap) = ctx.read_file(base.join("capacity")) {
                        let pct_str = cap.trim().to_string();
                        map.insert("percent_int".into(), pct_str.clone());
                        map.insert("percent".into(), format!("{}%", pct_str));
                    }
                    if let Ok(status) = ctx.read_file(base.join("status")) {
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

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::fs::{test_ctx, MockFs};

    #[test]
    fn reads_mock_bat0() {
        let ctx = test_ctx(
            MockFs::new()
                .file("/sys/class/power_supply/BAT0/capacity", "79\n")
                .file("/sys/class/power_supply/BAT0/status", "Discharging\n"),
        );
        let v = BatteryModule.collect(&ctx).unwrap();
        let map = match v {
            InfoValue::Map(m) => m,
            _ => panic!("expected map"),
        };
        assert_eq!(map.get("percent").map(String::as_str), Some("79%"));
        assert_eq!(map.get("percent_int").map(String::as_str), Some("79"));
        assert_eq!(map.get("status").map(String::as_str), Some("Discharging"));
    }

    #[test]
    fn unknown_when_no_battery() {
        let ctx = test_ctx(MockFs::new());
        assert!(matches!(
            BatteryModule.collect(&ctx).unwrap(),
            InfoValue::Scalar(ref s) if s == "unknown"
        ));
    }
}
