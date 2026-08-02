use crate::{Context, InfoValue, Module, Result};

pub struct KernelModule;

impl Module for KernelModule {
    fn name(&self) -> &'static str {
        "kernel"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        Ok(InfoValue::Scalar(kernel_string()))
    }
}

/// Zero-spawn kernel string (Phase 4.1: sub-10 ms cold start). On Linux the
/// uname data is available from procfs, so we never fork `uname`. The fallback
/// (macOS / non-Linux) still shells out.
fn kernel_string() -> String {
    #[cfg(target_os = "linux")]
    {
        let os = std::fs::read_to_string("/proc/sys/kernel/ostype")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Linux".into());
        let release = std::fs::read_to_string("/proc/sys/kernel/osrelease")
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        let machine = std::env::consts::ARCH;
        if !release.is_empty() {
            return format!("{os} {release} {machine}");
        }
    }

    std::process::Command::new("uname")
        .args(["-srm"])
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        })
        .unwrap_or_else(|| "unknown".to_string())
}
