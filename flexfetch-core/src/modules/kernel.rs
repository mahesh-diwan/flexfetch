use crate::{Context, InfoValue, Module, Result};

pub struct KernelModule;

impl Module for KernelModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        Ok(InfoValue::Scalar(kernel_string(ctx)))
    }
}

/// Zero-spawn kernel string (Phase 4.1: sub-10 ms cold start). On Linux the
/// uname data is available from procfs, so we never fork `uname`. The fallback
/// (macOS / non-Linux) still shells out.
#[allow(unused_variables)] // ctx is only read on Linux (macOS shells out)
fn kernel_string(ctx: &Context) -> String {
    #[cfg(target_os = "linux")]
    {
        let os = ctx
            .read_file("/proc/sys/kernel/ostype")
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|_| "Linux".into());
        let release = ctx
            .read_file("/proc/sys/kernel/osrelease")
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
