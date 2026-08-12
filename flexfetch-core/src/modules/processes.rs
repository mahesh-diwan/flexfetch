use crate::{Context, InfoValue, Module, Result};

pub struct ProcessesModule;

impl Module for ProcessesModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        let count = process_count(ctx);
        Ok(InfoValue::Scalar(count))
    }
}

#[allow(unused_variables)] // ctx only read on Linux
fn process_count(ctx: &Context) -> String {
    #[cfg(target_os = "linux")]
    {
        if let Ok(entries) = ctx.read_dir("/proc") {
            let count = entries
                .into_iter()
                .filter(|p| ctx.is_dir(p))
                .filter(|p| {
                    p.file_name()
                        .map(|s| s.to_string_lossy().bytes().all(|b| b.is_ascii_digit()))
                        .unwrap_or(false)
                })
                .count();
            return count.to_string();
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("ps").args(["-e"]).output() {
            let s = String::from_utf8_lossy(&output.stdout);
            let count = s.lines().count().saturating_sub(1); // header line
            return count.to_string();
        }
    }

    "unknown".to_string()
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::fs::{test_ctx, MockFs};

    #[test]
    fn counts_numeric_proc_dirs() {
        // PID dirs + one non-numeric entry that must be skipped.
        let ctx = test_ctx(
            MockFs::new()
                .dir("/proc/1")
                .dir("/proc/2")
                .dir("/proc/1234")
                .file("/proc/version", "mock\n"),
        );
        assert_eq!(process_count(&ctx), "3");
    }

    #[test]
    fn zero_when_proc_has_no_pids() {
        // An empty /proc is impossible on a real kernel, but MockFs shows the
        // collector degrades to "0" (read_dir succeeds with no numeric dirs).
        let ctx = test_ctx(MockFs::new().dir("/proc").file("/proc/version", "mock\n"));
        assert_eq!(process_count(&ctx), "0");
    }
}
