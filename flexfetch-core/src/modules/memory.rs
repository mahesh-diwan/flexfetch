use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct MemoryModule;

impl Module for MemoryModule {
    #[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = ctx.read_file("/proc/meminfo") {
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
                    // saturating: hostile /proc/meminfo with near-u64::MAX
                    // counters must not overflow the fallback sum.
                    avail_kb = free_kb.saturating_add(cached_kb);
                }

                if total_kb > 0 {
                    let used_kb = total_kb.saturating_sub(avail_kb);
                    let total_gb = total_kb as f64 / (1024.0 * 1024.0);
                    let used_gb = used_kb as f64 / (1024.0 * 1024.0);
                    let percent = (used_kb as f64 / total_kb as f64 * 100.0) as u32;

                    map.insert("total".into(), format!("{:.1} GiB", total_gb));
                    map.insert("used".into(), format!("{:.1} GiB", used_gb));
                    let percent_u8 = percent.min(100) as u8;
                    map.insert("percent_int".into(), percent_u8.to_string());
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
                        // u128 math: *100 overflows u64 near u64::MAX.
                        let pct = ((swap_used as u128 * 100 / swap_total as u128) as u64).min(100);
                        map.insert("swap_percent".into(), format!("{pct}%"));
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

                // saturating: a weird vm_stat dump must not overflow the sums.
                let used_pages = pages_active.saturating_add(pages_wired);
                let total_pages = used_pages.saturating_add(pages_free);
                if total_pages > 0 {
                    let total_bytes = total_pages.saturating_mul(page_size);
                    let used_bytes = used_pages.saturating_mul(page_size);
                    let total_gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    let used_gb = used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    let percent = (used_bytes as f64 / total_bytes as f64 * 100.0) as u32;

                    map.insert("total".into(), format!("{:.1} GiB", total_gb));
                    map.insert("used".into(), format!("{:.1} GiB", used_gb));
                    map.insert("percent".into(), format!("{}%", percent));
                }
            }
        }

        // Phase 8.9 — Windows: GlobalMemoryStatusEx (one syscall, no subprocess).
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::System::SystemInformation::{
                GlobalMemoryStatusEx, MEMORYSTATUSEX,
            };

            let mut ms: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
            ms.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
            if unsafe { GlobalMemoryStatusEx(&mut ms) } != 0 {
                let total = ms.ullTotalPhys;
                let used = total.saturating_sub(ms.ullAvailPhys);
                if total > 0 {
                    let percent = (used as f64 / total as f64 * 100.0).min(100.0) as u32;
                    map.insert(
                        "total".into(),
                        format!("{:.1} GiB", total as f64 / 1073741824.0),
                    );
                    map.insert(
                        "used".into(),
                        format!("{:.1} GiB", used as f64 / 1073741824.0),
                    );
                    map.insert("percent_int".into(), (percent.min(100) as u8).to_string());
                    map.insert("percent".into(), format!("{percent}%"));

                    // Page file ≈ swap.
                    let swap_total = ms.ullTotalPageFile;
                    let swap_used = swap_total.saturating_sub(ms.ullAvailPageFile);
                    if swap_total > 0 {
                        map.insert(
                            "swap_total".into(),
                            format!("{:.1} GiB", swap_total as f64 / 1073741824.0),
                        );
                        map.insert(
                            "swap_used".into(),
                            format!("{:.1} GiB", swap_used as f64 / 1073741824.0),
                        );
                        // u128 math: *100 overflows u64 near u64::MAX.
                        let pct = ((swap_used as u128 * 100 / swap_total as u128) as u64).min(100);
                        map.insert("swap_percent".into(), format!("{pct}%"));
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

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::fs::{test_ctx, MockFs};

    #[test]
    fn huge_meminfo_counters_do_not_overflow() {
        // free + cached both near u64::MAX: the fallback sum must saturate,
        // not panic in debug builds.
        let meminfo = "MemTotal:        1000000 kB\n\
                       MemFree:         18446744073709551615 kB\n\
                       Cached:          18446744073709551615 kB\n";
        let ctx = test_ctx(MockFs::new().file("/proc/meminfo", meminfo));
        match MemoryModule.collect(&ctx).unwrap() {
            InfoValue::Map(m) => {
                // used = total.saturating_sub(avail) -> 0
                assert_eq!(m.get("used").map(|s| s.as_str()), Some("0.0 GiB"));
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    #[test]
    fn huge_swap_percent_product_does_not_overflow() {
        // SwapTotal near u64::MAX, SwapFree 0: used*100 must stay exact (100%)
        // via u128 math — never overflow, never saturate down to 1%.
        let meminfo = "MemTotal:        1000000 kB\n\
                       SwapTotal:       18446744073709551615 kB\n\
                       SwapFree:        0 kB\n";
        let ctx = test_ctx(MockFs::new().file("/proc/meminfo", meminfo));
        match MemoryModule.collect(&ctx).unwrap() {
            InfoValue::Map(m) => {
                let pct = m
                    .get("swap_percent")
                    .map(|s| s.as_str())
                    .unwrap_or_default();
                // (MAX-0)*100/MAX = 100%
                assert_eq!(pct, "100%");
            }
            other => panic!("unexpected variant: {other:?}"),
        }
    }

    use proptest::prelude::*;

    proptest! {
        /// Arbitrary /proc/meminfo must never panic the collector, and any
        /// rendered percent must be within 0..=100.
        #[test]
        fn collect_never_panics_on_arbitrary_meminfo(content in ".*") {
            let ctx = test_ctx(MockFs::new().file("/proc/meminfo", &content));
            if let InfoValue::Map(m) = MemoryModule.collect(&ctx).unwrap() {
                for key in ["percent", "swap_percent"] {
                    if let Some(pct) = m.get(key).and_then(|s| {
                        s.strip_suffix('%').and_then(|n| n.parse::<u32>().ok())
                    }) {
                        prop_assert!(pct <= 100, "{key} {pct} out of range");
                    }
                }
            }
        }
    }
}
