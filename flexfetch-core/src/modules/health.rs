use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct HealthModule;

fn read_u64_file(ctx: &Context, path: &str) -> Option<u64> {
    ctx.read_file(path).ok()?.trim().parse::<u64>().ok()
}

/// Disk usage % for the root filesystem via libc::statvfs (no `df` subprocess).
/// POSIX only (Windows has no statvfs; the health module degrades to the
/// metrics that exist there).
#[cfg(unix)]
fn disk_usage_percent() -> Option<u8> {
    let c = std::ffi::CString::new("/").ok()?;
    let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(c.as_ptr(), &mut st) } != 0 {
        return None;
    }
    let frsize = st.f_frsize as u64;
    let total = st.f_blocks as u64 * frsize;
    let avail = st.f_bavail as u64 * frsize;
    if total == 0 {
        return None;
    }
    let pct = ((total - avail) * 100 / total).min(100) as u8;
    Some(pct)
}

/// Swap usage % from /proc/meminfo (None when no swap).
fn swap_percent(ctx: &Context) -> Option<u8> {
    let content = ctx.read_file("/proc/meminfo").ok()?;
    let mut total = 0u64;
    let mut free = 0u64;
    for line in content.lines() {
        if let Some((k, v)) = line.split_once(':') {
            let num: u64 = v.trim().trim_end_matches(" kB").parse().ok()?;
            match k.trim() {
                "SwapTotal" => total = num,
                "SwapFree" => free = num,
                _ => {}
            }
        }
    }
    if total == 0 {
        return None;
    }
    Some(((total.saturating_sub(free)) * 100 / total).min(100) as u8)
}

/// 1-minute load average normalized per logical core (1.0 = saturated).
fn load_per_core(ctx: &Context) -> Option<f64> {
    let content = ctx.read_file("/proc/loadavg").ok()?;
    let load: f64 = content.split_whitespace().next()?.parse().ok()?;
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    Some(load / cores as f64)
}

/// Battery % (100 when charging/full so it never penalizes a plugged-in machine).
fn battery_percent(ctx: &Context) -> Option<u8> {
    let entries = ctx.read_dir("/sys/class/power_supply").ok()?;
    for entry in entries {
        let name = entry
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if name.starts_with("BAT") {
            let status = ctx.read_file(entry.join("status")).unwrap_or_default();
            let pct: u8 =
                read_u64_file(ctx, &entry.join("capacity").to_string_lossy())?.min(100) as u8;
            let charging = matches!(status.trim(), "Charging" | "Full");
            return Some(if charging { 100 } else { pct });
        }
    }
    None
}

impl Module for HealthModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();
        let mut score: i32 = 100;
        let mut notes: Vec<String> = Vec::new();

        #[cfg(unix)]
        if let Some(pct) = disk_usage_percent() {
            map.insert("disk_pct".into(), format!("{pct}%"));
            if pct > 90 {
                score -= (pct - 90) as i32 * 2;
                notes.push(format!("disk {pct}%"));
            }
        }
        if let Some(pct) = swap_percent(ctx) {
            map.insert("swap_pct".into(), format!("{pct}%"));
            if pct > 50 {
                score -= (pct - 50) as i32 / 2;
                notes.push(format!("swap {pct}%"));
            }
        }
        if let Some(lpc) = load_per_core(ctx) {
            map.insert("load".into(), format!("{lpc:.2}"));
            if lpc > 1.0 {
                score -= ((lpc - 1.0) * 20.0) as i32;
                notes.push(format!("load {lpc:.2}/core"));
            }
        }
        if let Some(pct) = battery_percent(ctx) {
            map.insert("battery_pct".into(), format!("{pct}%"));
            if pct < 80 {
                score -= (80 - pct) as i32 / 2;
                notes.push(format!("battery {pct}%"));
            }
        }

        let score = score.clamp(0, 100) as u8;
        let grade = match score {
            90..=100 => "Excellent",
            75..=89 => "Good",
            50..=74 => "Fair",
            _ => "Poor",
        };
        map.insert("score".into(), score.to_string());
        map.insert("grade".into(), grade.to_string());
        if !notes.is_empty() {
            map.insert("notes".into(), notes.join(", "));
        }

        Ok(InfoValue::Map(map))
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::fs::{test_ctx, MockFs};

    #[test]
    fn swap_percent_from_mock_meminfo() {
        let meminfo = "MemTotal:        16000000 kB\n\
                       SwapTotal:        8000000 kB\n\
                       SwapFree:         4000000 kB\n";
        let ctx = test_ctx(MockFs::new().file("/proc/meminfo", meminfo));
        assert_eq!(swap_percent(&ctx), Some(50));
    }

    #[test]
    fn swap_none_without_swap() {
        let meminfo = "MemTotal:        16000000 kB\n\
                       SwapTotal:        0 kB\n\
                       SwapFree:         0 kB\n";
        let ctx = test_ctx(MockFs::new().file("/proc/meminfo", meminfo));
        assert_eq!(swap_percent(&ctx), None);
    }

    #[test]
    fn load_per_core_parses() {
        let ctx = test_ctx(MockFs::new().file("/proc/loadavg", "2.00 1.50 1.00 2/123 4567\n"));
        let lpc = load_per_core(&ctx).expect("load should parse");
        assert!(lpc > 0.0);
    }
}
