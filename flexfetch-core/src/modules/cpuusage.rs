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
    // saturating fold: a plain sum() panics in debug (and wraps in release)
    // on counters near u64::MAX — a hostile/malformed /proc/stat must not
    // crash the fetch.
    let total: u64 = parts
        .iter()
        .skip(1)
        .filter_map(|v| v.parse::<u64>().ok())
        .fold(0u64, u64::saturating_add);
    let idle: u64 = parts
        .get(4)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0u64)
        .saturating_add(parts.get(5).and_then(|v| v.parse().ok()).unwrap_or(0u64));
    if total == 0 {
        return None;
    }
    // saturating_sub: malformed /proc/stat (idle > total) must not panic in
    // debug or wrap in release — a garbage file reports 0% busy, not UB.
    Some(total.saturating_sub(idle) as f64 / total as f64 * 100.0)
}

#[cfg(all(test, target_os = "linux"))]
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

    #[test]
    fn missing_cpu_line_is_none() {
        assert!(since_boot_usage("cpu0 1 2 3 4 5\n").is_none());
        // Only a per-core line, no aggregate `cpu ` line.
        assert!(since_boot_usage("cpu0 1 2 3 4 5\ncpu1 1 2 3 4 5\n").is_none());
    }

    #[test]
    fn huge_values_saturate_safely() {
        // Near-u64::MAX counters: a plain `sum()` panics in debug on this
        // input; the saturating fold keeps the result a finite percentage.
        let content = "cpu 18446744073709551615 18446744073709551615 0 0 0 0 0 0 0 0";
        let pct = since_boot_usage(content).unwrap();
        assert!((0.0..=100.0).contains(&pct), "pct out of range: {pct}");
    }

    use proptest::prelude::*;

    proptest! {
        /// The /proc/stat parser must never panic and must always return a
        /// sane percentage on arbitrary (hostile/malformed) input.
        #[test]
        fn since_boot_usage_never_panics(content in ".*") {
            if let Some(pct) = since_boot_usage(&content) {
                prop_assert!(
                    (0.0..=100.0).contains(&pct),
                    "pct out of range: {pct}"
                );
            }
        }

        /// Any two whitespace-separated numbers, even near u64::MAX, must
        /// produce a finite percentage — never an overflow panic.
        #[test]
        fn since_boot_usage_huge_fields(rest in prop::collection::vec("[0-9]{1,20}", 0..20)) {
            let content = format!("cpu {}", rest.join(" "));
            if let Some(pct) = since_boot_usage(&content) {
                prop_assert!((0.0..=100.0).contains(&pct));
            }
        }
    }
}
