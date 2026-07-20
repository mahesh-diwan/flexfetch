use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct MemoryModule;

impl Module for MemoryModule {
    fn name(&self) -> &'static str { "memory" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
                let mut total_kb = 0u64;
                let mut avail_kb = 0u64;
                for line in content.lines() {
                    if let Some(val) = line.strip_prefix("MemTotal:") {
                        total_kb = parse_kb(val);
                    }
                    if let Some(val) = line.strip_prefix("MemAvailable:") {
                        avail_kb = parse_kb(val);
                    }
                }
                let used_kb = total_kb.saturating_sub(avail_kb);
                map.insert("total".into(), fmt_kb(total_kb));
                map.insert("used".into(), fmt_kb(used_kb));
                map.insert("available".into(), fmt_kb(avail_kb));
                if total_kb > 0 {
                    map.insert("percent".into(), format!("{:.0}%", used_kb as f64 / total_kb as f64 * 100.0));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let total_kb = sysctl_u64("hw.memsize").unwrap_or(0) / 1024;
            if let Ok(out) = std::process::Command::new("vm_stat").output() {
                let s = String::from_utf8_lossy(&out.stdout);
                let page_size = sysctl_u64("hw.pagesize").unwrap_or(4096);
                let mut active = 0u64; let mut wired = 0u64;
                for line in s.lines() {
                    if let Some(v) = line.strip_prefix("Pages active:") {
                        active = v.trim().trim_end_matches('.').parse().unwrap_or(0);
                    }
                    if let Some(v) = line.strip_prefix("Pages wired down:") {
                        wired = v.trim().trim_end_matches('.').parse().unwrap_or(0);
                    }
                }
                let used_kb = ((active + wired) * page_size) / 1024;
                map.insert("total".into(), fmt_kb(total_kb));
                map.insert("used".into(), fmt_kb(used_kb));
                let avail = total_kb.saturating_sub(used_kb);
                map.insert("available".into(), fmt_kb(avail));
                if total_kb > 0 {
                    map.insert("percent".into(), format!("{:.0}%", used_kb as f64 / total_kb as f64 * 100.0));
                }
            } else {
                map.insert("total".into(), fmt_kb(total_kb));
            }
        }

        Ok(InfoValue::Map(map))
    }
}

#[cfg(target_os = "linux")]
fn parse_kb(s: &str) -> u64 {
    s.split_whitespace().next().and_then(|n| n.parse::<u64>().ok()).unwrap_or(0)
}

fn fmt_kb(kb: u64) -> String {
    if kb >= 1024 * 1024 {
        format!("{:.2} GiB", kb as f64 / (1024.0 * 1024.0))
    } else if kb >= 1024 {
        format!("{:.2} MiB", kb as f64 / 1024.0)
    } else {
        format!("{kb} KiB")
    }
}

#[cfg(target_os = "macos")]
fn sysctl_u64(name: &str) -> Option<u64> {
    use std::ffi::CString;
    let cname = CString::new(name).ok()?;
    let mut val: u64 = 0;
    let mut size = std::mem::size_of::<u64>();
    if unsafe { libc::sysctlbyname(cname.as_ptr(), &mut val as *mut _ as *mut std::ffi::c_void, &mut size, std::ptr::null_mut(), 0) } == 0 {
        Some(val)
    } else {
        None
    }
}
