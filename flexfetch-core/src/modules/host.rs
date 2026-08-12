use crate::{Context, InfoValue, Module, Result};

pub struct HostModule;

impl Module for HostModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        Ok(InfoValue::Scalar(
            hostname(ctx).unwrap_or_else(|| "unknown".into()),
        ))
    }
}

#[allow(unused_variables)] // ctx is only read on Linux (macOS uses libc)
fn hostname(ctx: &Context) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        ctx.read_file("/proc/sys/kernel/hostname")
            .ok()
            .map(|s| s.trim().to_string())
    }
    #[cfg(target_os = "macos")]
    {
        let mut buf = vec![0u8; 256];
        if unsafe { libc::gethostname(buf.as_mut_ptr() as *mut std::ffi::c_char, 255) } == 0 {
            let len = buf.iter().position(|&c| c == 0).unwrap_or(0);
            Some(
                std::str::from_utf8(&buf[..len])
                    .unwrap_or("mac")
                    .to_string(),
            )
        } else {
            None
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::fs::{test_ctx, MockFs};

    #[test]
    fn hostname_reads_mock_proc() {
        let ctx = test_ctx(MockFs::new().file("/proc/sys/kernel/hostname", "mybox\n"));
        assert_eq!(hostname(&ctx).as_deref(), Some("mybox"));
    }

    #[test]
    fn hostname_empty_when_file_missing() {
        let ctx = test_ctx(MockFs::new());
        assert!(hostname(&ctx).is_none());
    }
}
