use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct TemperatureModule;

impl Module for TemperatureModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        // Try /sys/class/thermal first (most reliable)
        if let Ok(entries) = ctx.read_dir("/sys/class/thermal") {
            for path in entries {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if name.starts_with("thermal_zone") {
                    if let Ok(temp_str) = ctx.read_file(path.join("temp")) {
                        if let Ok(mk) = temp_str.trim().parse::<u64>() {
                            map.insert("cpu".into(), format!("{}°C", mk / 1000));
                            break;
                        }
                    }
                }
            }
        }

        // GPU temp from hwmon
        if let Ok(drm) = ctx.read_dir("/sys/class/drm") {
            for path in drm {
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if !name.starts_with("card") {
                    continue;
                }
                let hwmon_base = path.join("device/hwmon");
                if let Ok(hm_entries) = ctx.read_dir(&hwmon_base) {
                    for hm in hm_entries {
                        let temp_file = hm.join("temp1_input");
                        if let Ok(temp_str) = ctx.read_file(&temp_file) {
                            if let Ok(mk) = temp_str.trim().parse::<u64>() {
                                map.insert("gpu".into(), format!("{}°C", mk / 1000));
                                break;
                            }
                        }
                    }
                    if map.contains_key("gpu") {
                        break;
                    }
                }
            }
        }

        // Fan speed from hwmon
        if let Ok(hwmon) = ctx.read_dir("/sys/class/hwmon") {
            for path in hwmon {
                let fan_file = path.join("fan1_input");
                if let Ok(fan_str) = ctx.read_file(&fan_file) {
                    if let Ok(rpm) = fan_str.trim().parse::<u64>() {
                        if rpm > 0 {
                            map.insert("fan".into(), format!("{} RPM", rpm));
                            break;
                        }
                    }
                }
            }
        }

        // Fallback: try `sensors` command
        if map.is_empty() {
            if let Ok(output) = std::process::Command::new("sensors").output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let trimmed = line.trim();
                    if trimmed.contains("temp1:") && !map.contains_key("cpu") {
                        if let Some(val) = trimmed.split("temp1:").nth(1) {
                            let val = val.split_whitespace().next().unwrap_or("");
                            if !val.is_empty() {
                                map.insert("cpu".into(), format!("{}°C", val));
                            }
                        }
                    } else if trimmed.contains("fan1:") && !map.contains_key("fan") {
                        if let Some(val) = trimmed.split("fan1:").nth(1) {
                            let val = val.split_whitespace().next().unwrap_or("");
                            if !val.is_empty() {
                                map.insert("fan".into(), format!("{} RPM", val));
                            }
                        }
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
