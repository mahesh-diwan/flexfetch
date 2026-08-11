use crate::{Context, InfoValue, Module, Result};

pub struct CpuUsageModule;

impl Module for CpuUsageModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            let content = _ctx.read_file("/proc/stat").unwrap_or_default();
            let usage = since_boot_usage(&content).unwrap_or(0.0);
            Ok(InfoValue::Scalar(format!("{usage:.1}%")))
        }
        #[cfg(not(target_os = "linux"))]
        {
            Ok(InfoValue::Scalar("unknown".into()))
        }
    }
}

/// Busy percentage since boot from a `/proc/stat` dump: one read, no sampling window.
/// Idle = idle + iowait; both count toward the non-busy total.
#[cfg(target_os = "linux")]
fn since_boot_usage(content: &str) -> Option<f64> {
    let line = content.lines().find(|l| l.starts_with("cpu "))?;
    let parts: Vec<&str> = line.split_whitespace().collect();
    let total: u64 = parts
        .iter()
        .skip(1)
        .filter_map(|v| v.parse::<u64>().ok())
        .sum();
    let idle: u64 = parts.get(4).and_then(|v| v.parse().ok()).unwrap_or(0)
        + parts.get(5).and_then(|v| v.parse().ok()).unwrap_or(0);
    if total == 0 {
        return None;
    }
    Some((total - idle) as f64 / total as f64 * 100.0)
}

#[cfg(test)]
mod tests {
    use super::since_boot_usage;

    #[test]
    fn idle_equals_total_is_nearly_zero() {
        // total = 1001, idle = 1000 + 1 (iowait) => 0% busy
        let content = "cpu 0 0 0 1000 1 0 0 0 0 0";
        let pct = since_boot_usage(content).unwrap();
        assert!(pct < 0.01, "expected ~0%, got {pct}");
    }

    #[test]
    fn half_idle_is_fifty_percent() {
        let content = "cpu 100 0 0 100 0 0 0 0 0 0";
        let pct = since_boot_usage(content).unwrap();
        assert!((pct - 50.0).abs() < 0.01, "expected 50%, got {pct}");
    }

    #[test]
    fn garbage_input_is_none() {
        assert!(since_boot_usage("").is_none());
        assert!(since_boot_usage("not a stat file").is_none());
    }
}
