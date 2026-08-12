use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct SwapModule;

impl Module for SwapModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = ctx.read_file("/proc/swaps") {
                let mut total_kb = 0u64;
                let mut used_kb = 0u64;

                // Header: "Filename\tType\tSize\tUsed\tPriority"
                for line in content.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        if let (Ok(size), Ok(used)) =
                            (parts[2].parse::<u64>(), parts[3].parse::<u64>())
                        {
                            // saturating: a malformed /proc/swaps with huge
                            // counters must not overflow the accumulation.
                            total_kb = total_kb.saturating_add(size);
                            used_kb = used_kb.saturating_add(used);
                        }
                    }
                }

                if total_kb > 0 {
                    let total_gb = total_kb as f64 / 1048576.0;
                    let used_gb = used_kb as f64 / 1048576.0;
                    // Clamp: malformed /proc/swaps (used > size) must not
                    // render "147%".
                    let percent = ((used_kb as f64 / total_kb as f64 * 100.0) as u32).min(100);

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

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::fs::{test_ctx, MockFs};

    fn percent_of(content: &str) -> String {
        let ctx = test_ctx(MockFs::new().file("/proc/swaps", content));
        match SwapModule.collect(&ctx).unwrap() {
            InfoValue::Map(m) => m.get("percent").cloned().unwrap_or_default(),
            _ => String::new(),
        }
    }

    #[test]
    fn parses_swap_entries() {
        let content = "Filename\tType\tSize\tUsed\tPriority\n\
                       /dev/zram0\tpartition\t4194304\t1048576\t-2\n";
        // 1048576 / 4194304 = 25%
        assert_eq!(percent_of(content), "25%");
    }

    #[test]
    fn percent_clamped_when_used_exceeds_size() {
        // Malformed file (used > size): must clamp to 100%, never "147%".
        let content = "Filename\tType\tSize\tUsed\tPriority\n\
                       /dev/zram0\tpartition\t4194304\t6160384\t-2\n";
        assert_eq!(percent_of(content), "100%");
    }

    #[test]
    fn skips_garbage_lines() {
        // The middle line has enough whitespace tokens to pass the len guard,
        // but its size/used fields don't parse as u64 — must be skipped, not
        // abort or overflow the accumulation.
        let content = "Filename\tType\tSize\tUsed\tPriority\n\
                       not a swap line at all\n\
                       /dev/zram0\tpartition\t4194304\t1048576\t-2\n";
        assert_eq!(percent_of(content), "25%");
    }

    #[test]
    fn no_swap_returns_unknown_or_fallback() {
        let ctx = test_ctx(MockFs::new());
        match SwapModule.collect(&ctx).unwrap() {
            InfoValue::Scalar(_) | InfoValue::Map(_) => {}
            _ => panic!("unexpected variant"),
        }
    }
}
