use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct SwapModule;

impl Module for SwapModule {
    fn name(&self) -> &'static str {
        "swap"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/swaps") {
                let mut total_kb = 0u64;
                let mut used_kb = 0u64;

                // Header: "Filename\tType\tSize\tUsed\tPriority"
                for line in content.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        if let (Ok(size), Ok(used)) =
                            (parts[2].parse::<u64>(), parts[3].parse::<u64>())
                        {
                            total_kb += size;
                            used_kb += used;
                        }
                    }
                }

                if total_kb > 0 {
                    let total_gb = total_kb as f64 / 1048576.0;
                    let used_gb = used_kb as f64 / 1048576.0;
                    let percent = (used_kb as f64 / total_kb as f64 * 100.0) as u32;

                    let mut map = HashMap::new();
                    map.insert("total".into(), format!("{:.1} GiB", total_gb));
                    map.insert("used".into(), format!("{:.1} GiB", used_gb));
                    map.insert("percent".into(), format!("{}%", percent));
                    return Ok(InfoValue::Map(map));
                }
            }
        }

        // Fallback: free -h
        if let Ok(output) = std::process::Command::new("free").arg("-h").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Some(rest) = line.strip_prefix("Swap:") {
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let mut map = HashMap::new();
                        map.insert("total".into(), parts[0].to_string());
                        map.insert("used".into(), parts[1].to_string());
                        if parts.len() >= 3 {
                            map.insert("percent".into(), parts[2].to_string());
                        }
                        return Ok(InfoValue::Map(map));
                    }
                }
            }
        }

        Ok(InfoValue::Scalar("unknown".into()))
    }
}
