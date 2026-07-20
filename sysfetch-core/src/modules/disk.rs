use crate::{Module, InfoValue, Context, Result};

pub struct DiskModule;

impl Module for DiskModule {
    fn name(&self) -> &'static str { "disk" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mounts = get_mounts().unwrap_or_default();
        Ok(InfoValue::List(mounts))
    }
}

fn get_mounts() -> Option<Vec<String>> {
    let mut result = Vec::new();

    #[cfg(target_os = "linux")]
    {
        let content = std::fs::read_to_string("/proc/mounts").ok()?;
        let mut seen = std::collections::HashSet::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 { continue; }
            let mount_point = parts[1];
            if mount_point.starts_with("/sys") || mount_point.starts_with("/proc")
                || mount_point.starts_with("/dev") || mount_point.starts_with("/run")
                || mount_point == "/" { continue; }
            if mount_point.starts_with("/") && seen.insert(mount_point.to_string()) {
                if let Some(usage) = mount_usage(mount_point) {
                    result.push(format!("{mount_point} {usage}"));
                }
            }
        }
        if let Some(usage) = mount_usage("/") {
            result.insert(0, format!("/ {usage}"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(out) = std::process::Command::new("df").args(["-H"]).output() {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let mount = parts[5];
                    if mount.starts_with("/") {
                        result.push(format!("{mount} {used}/{size}", used=parts[2], size=parts[1]));
                    }
                }
            }
        }
    }

    if result.is_empty() { None } else { Some(result) }
}

fn mount_usage(path: &str) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
        let cpath = std::ffi::CString::new(path).ok()?;
        if unsafe { libc::statvfs(cpath.as_ptr(), &mut stat) } != 0 {
            return None;
        }
        let total = stat.f_blocks as u64 * stat.f_frsize as u64;
        let used = total.saturating_sub(stat.f_bfree as u64 * stat.f_frsize as u64);
        let pct = if total > 0 { format!("{:.0}%", used as f64 / total as f64 * 100.0) } else { "?".into() };
        Some(format!("{}/{} ({})", bytes_fmt(used), bytes_fmt(total), pct))
    }
    #[cfg(not(target_os = "linux"))]
    { None }
}

fn bytes_fmt(bytes: u64) -> String {
    if bytes >= 1 << 40 {
        format!("{:.1}TiB", bytes as f64 / (1u64 << 40) as f64)
    } else if bytes >= 1 << 30 {
        format!("{:.1}GiB", bytes as f64 / (1u64 << 30) as f64)
    } else if bytes >= 1 << 20 {
        format!("{:.1}MiB", bytes as f64 / (1u64 << 20) as f64)
    } else {
        format!("{bytes}B")
    }
}
