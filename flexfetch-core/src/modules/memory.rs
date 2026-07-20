use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct MemoryModule;

impl Module for MemoryModule {
    fn name(&self) -> &'static str {
        "memory"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
                let mut total_kb = 0u64;
                let mut avail_kb = 0u64;
                let mut free_kb = 0u64;
                let mut cached_kb = 0u64;
                let mut swap_total = 0u64;
                let mut swap_free = 0u64;

                for line in content.lines() {
                    if let Some((key, val)) = line.split_once(':') {
                        let val = val.trim().trim_end_matches(" kB");
                        if let Ok(num) = val.parse::<u64>() {
                            match key.trim() {
                                "MemTotal" => total_kb = num,
                                "MemAvailable" => avail_kb = num,
                                "MemFree" => free_kb = num,
                                "Cached" => cached_kb = num,
                                "SwapTotal" => swap_total = num,
                                "SwapFree" => swap_free = num,
                                _ => {}
                            }
                        }
                    }
                }

                if avail_kb == 0 {
                    avail_kb = free_kb + cached_kb;
                }

                if total_kb > 0 {
                    let used_kb = total_kb.saturating_sub(avail_kb);
                    let total_gb = total_kb as f64 / (1024.0 * 1024.0);
                    let used_gb = used_kb as f64 / (1024.0 * 1024.0);
                    let percent = (used_kb as f64 / total_kb as f64 * 100.0) as u32;

                    map.insert("total".into(), format!("{:.1} GiB", total_gb));
                    map.insert("used".into(), format!("{:.1} GiB", used_gb));
                    map.insert("percent".into(), format!("{}%", percent));

                    if swap_total > 0 {
                        let swap_used = swap_total.saturating_sub(swap_free);
                        map.insert(
                            "swap_total".into(),
                            format!("{:.1} GiB", swap_total as f64 / 1048576.0),
                        );
                        map.insert(
                            "swap_used".into(),
                            format!("{:.1} GiB", swap_used as f64 / 1048576.0),
                        );
                        map.insert(
                            "swap_percent".into(),
                            format!("{}%", swap_used * 100 / swap_total),
                        );
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("vm_stat").output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut page_size = 4096u64;
                let mut pages_free = 0u64;
                let mut pages_active = 0u64;
                let mut pages_wired = 0u64;

                if let Ok(ps) = std::process::Command::new("sysctl")
                    .args(["-n", "hw.pagesize"])
                    .output()
                {
                    let s = String::from_utf8_lossy(&ps.stdout).trim().to_string();
                    page_size = s.parse().unwrap_or(4096);
                }

                for line in stdout.lines() {
                    if let Some((key, val)) = line.split_once(':') {
                        let val = val.trim().trim_end_matches('.');
                        if let Ok(num) = val.parse::<u64>() {
                            match key.trim() {
                                "Pages free" => pages_free = num,
                                "Pages active" => pages_active = num,
                                "Pages wired down" => pages_wired = num,
                                _ => {}
                            }
                        }
                    }
                }

                let used_pages = pages_active + pages_wired;
                let total_pages = used_pages + pages_free;
                if total_pages > 0 {
                    let total_bytes = total_pages * page_size;
                    let used_bytes = used_pages * page_size;
                    let total_gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    let used_gb = used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    let percent = (used_bytes as f64 / total_bytes as f64 * 100.0) as u32;

                    map.insert("total".into(), format!("{:.1} GiB", total_gb));
                    map.insert("used".into(), format!("{:.1} GiB", used_gb));
                    map.insert("percent".into(), format!("{}%", percent));
                }
            }
        }

        if map.is_empty() {
            return Ok(InfoValue::Scalar("unknown".into()));
        }
        Ok(InfoValue::Map(map))
    }
}
