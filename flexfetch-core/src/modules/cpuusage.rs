use crate::{Context, InfoValue, Module, Result};
use std::time::{Duration, Instant};

pub struct CpuUsageModule;

impl Module for CpuUsageModule {
    fn name(&self) -> &'static str {
        "cpuusage"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            let usage = read_usage().unwrap_or(0.0);
            Ok(InfoValue::Scalar(format!("{usage:.1}%")))
        }
        #[cfg(not(target_os = "linux"))]
        {
            Ok(InfoValue::Scalar("unknown".into()))
        }
    }
}

#[cfg(target_os = "linux")]
fn read_usage() -> Option<f64> {
    let snapshot = || -> Option<(u64, u64)> {
        let content = std::fs::read_to_string("/proc/stat").ok()?;
        let line = content.lines().next()?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let total: u64 = parts
            .iter()
            .skip(1)
            .filter_map(|v| v.parse::<u64>().ok())
            .sum();
        let idle: u64 = parts.get(4).and_then(|v| v.parse().ok()).unwrap_or(0);
        Some((total, idle))
    };

    let (t1, i1) = snapshot()?;
    let _start = Instant::now();
    std::thread::sleep(Duration::from_millis(100));
    let (t2, i2) = snapshot()?;

    let dt = t2.saturating_sub(t1);
    let di = i2.saturating_sub(i1);
    if dt == 0 {
        return None;
    }
    Some((dt - di) as f64 / dt as f64 * 100.0)
}
