use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct CpuModule;

impl Module for CpuModule {
    fn name(&self) -> &'static str {
        "cpu"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
                let mut cores = 0u32;
                for line in content.lines() {
                    if let Some((key, val)) = line.split_once(':') {
                        let key = key.trim();
                        let val = val.trim();
                        match key {
                            "model name" if !map.contains_key("model") => {
                                map.insert("model".into(), val.to_string());
                            }
                            "processor" => {
                                cores += 1;
                            }
                            "cpu MHz" if !map.contains_key("freq_mhz") => {
                                map.insert("freq_mhz".into(), val.to_string());
                            }
                            _ => {}
                        }
                    }
                }
                map.insert("cores".into(), cores.to_string());
            }

            // CPU usage: single read without sleep (fast but no usage percentage)
            if let Ok(content) = std::fs::read_to_string("/proc/stat") {
                if let Some(line) = content.lines().next() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    let total: u64 = parts
                        .iter()
                        .skip(1)
                        .filter_map(|v| v.parse::<u64>().ok())
                        .sum();
                    let idle: u64 = parts.get(4).and_then(|v| v.parse().ok()).unwrap_or(0);
                    map.insert("total_ticks".into(), total.to_string());
                    map.insert("idle_ticks".into(), idle.to_string());
                    // Usage calculation requires two samples over time - skipped for speed
                }
            }

            // CPU temp
            if let Ok(entries) = std::fs::read_dir("/sys/class/thermal") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("thermal_zone") {
                        if let Ok(temp_str) = std::fs::read_to_string(entry.path().join("temp")) {
                            if let Ok(mk) = temp_str.trim().parse::<u64>() {
                                map.insert("temp".into(), format!("{}°C", mk / 1000));
                                break;
                            }
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("sysctl")
                .args(["-n", "hw.model"])
                .output()
            {
                let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !v.is_empty() {
                    map.insert("model".into(), v);
                }
            }
            if let Ok(output) = std::process::Command::new("sysctl")
                .args(["-n", "hw.logicalcpu"])
                .output()
            {
                let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !v.is_empty() {
                    map.insert("cores".into(), v);
                }
            }
            if let Ok(output) = std::process::Command::new("sysctl")
                .args(["-n", "hw.cpufrequency"])
                .output()
            {
                let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !v.is_empty() {
                    let mhz = v.parse::<f64>().ok().map(|h| h / 1_000_000.0);
                    if let Some(mhz) = mhz {
                        map.insert("freq_mhz".into(), format!("{:.0}", mhz));
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
