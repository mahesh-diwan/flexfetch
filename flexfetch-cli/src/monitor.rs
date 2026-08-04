//! Shared health sampling for Phase 5.5 (history) + 5.6 (notifications).
//!
//! Collects the four health metrics (cpu %, mem %, disk %, temp °C) using the
//! existing module collectors via the registry — zero new system knowledge, and
//! the collectors are all zero-spawn on Linux (Phase 4.1).

use flexfetch_core::{Context, InfoValue, ModuleRegistry};

/// One health snapshot. `None` = collector unavailable (non-Linux, feature-off
/// module, or transient read failure).
#[derive(Debug, Clone, Copy, Default)]
pub struct Health {
    pub cpu_pct: Option<f64>,
    pub mem_pct: Option<u8>,
    pub disk_pct: Option<u8>,
    pub temp_c: Option<f64>,
}

impl Health {
    /// Any metric present at all? (Used to skip DB rows / notifications when
    /// nothing could be collected, e.g. `sensors` absent on a container.)
    pub fn any(&self) -> bool {
        self.cpu_pct.is_some()
            || self.mem_pct.is_some()
            || self.disk_pct.is_some()
            || self.temp_c.is_some()
    }
}

/// Sample all four health metrics. `cpuusage` sleeps ~30 ms internally (its
/// sampling window); the others are file reads.
pub fn sample_health(ctx: &Context) -> Health {
    let registry = ModuleRegistry::get();

    let cpu_pct = match registry.run_individual("cpuusage", ctx) {
        Some(InfoValue::Scalar(s)) => s.trim_end_matches('%').parse::<f64>().ok(),
        _ => None,
    };

    let mem_pct = match registry.run_individual("memory", ctx) {
        Some(InfoValue::Map(m)) => m.get("percent_int").and_then(|v| v.parse::<u8>().ok()),
        _ => None,
    };

    // Disk % from the disk collector's first entry ("/ ... 67%"); falls back to
    // a direct statvfs probe so containers without a matched mount still work.
    let disk_pct = match registry.run_individual("disk", ctx) {
        Some(InfoValue::List(list)) => list.iter().find_map(|e| {
            e.rsplit(' ')
                .next()
                .and_then(|p| p.trim_end_matches('%').parse::<u8>().ok())
        }),
        _ => None,
    }
    .or_else(disk_pct_statvfs);

    let temp_c = match registry.run_individual("temperature", ctx) {
        Some(InfoValue::Map(m)) => m.get("cpu").and_then(|v| parse_temp_c(v)),
        _ => None,
    };

    Health {
        cpu_pct,
        mem_pct,
        disk_pct,
        temp_c,
    }
}

/// Parse a temperature string like `"85°C"` (or `"85°c"`) into a number.
/// Trims any trailing °/C/c in any order so `85°C` doesn't get stuck at `85°`,
/// and tolerates a space before the unit (`"90 C"`).
fn parse_temp_c(s: &str) -> Option<f64> {
    s.trim()
        .trim_end_matches(['°', 'C', 'c'])
        .trim()
        .parse::<f64>()
        .ok()
}

/// Direct `statvfs` on `/` as a fallback when the disk collector's list parsing
/// finds nothing parseable.
fn disk_pct_statvfs() -> Option<u8> {
    #[cfg(unix)]
    {
        use std::ffi::CString;
        let c = CString::new("/").ok()?;
        let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
        if unsafe { libc::statvfs(c.as_ptr(), &mut st) } != 0 {
            return None;
        }
        let total = st.f_blocks as u64 * st.f_frsize as u64;
        let avail = st.f_bavail as u64 * st.f_frsize as u64;
        if total == 0 {
            return None;
        }
        Some(((total - avail) * 100 / total).min(100) as u8)
    }
    #[cfg(not(unix))]
    {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cpu_scalar() {
        let s = "37.1%".to_string();
        assert_eq!(s.trim_end_matches('%').parse::<f64>().ok(), Some(37.1));
    }

    #[test]
    fn parse_temp_string() {
        for s in ["85°C", "85°c", "85.5°C", "90 C"] {
            let v = parse_temp_c(s);
            assert!(v.is_some(), "failed to parse {s:?}");
            assert!(v.unwrap() > 0.0);
        }
        assert_eq!(parse_temp_c("abc"), None);
    }

    #[test]
    fn parse_disk_list_entry() {
        let entry = "/: 1.9G / 1.2G 67%";
        let pct = entry
            .rsplit(' ')
            .next()
            .and_then(|p| p.trim_end_matches('%').parse::<u8>().ok());
        assert_eq!(pct, Some(67));
    }
}
