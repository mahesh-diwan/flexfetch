use crate::{Context, InfoValue, Module, Result};

/// Init system — systemd/OpenRC/others, detected without spawning processes.
pub struct InitsystemModule;

impl Module for InitsystemModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            // systemd: the well-known marker dir.
            if ctx.is_dir("/run/systemd/system") || ctx.is_dir("/run/systemd") {
                return Ok(InfoValue::Scalar("systemd".into()));
            }
            // PID 1's comm name is the most reliable generic signal.
            if let Ok(comm) = ctx.read_file("/proc/1/comm") {
                let name = comm.trim().to_string();
                if !name.is_empty() && name != "unknown" {
                    return Ok(InfoValue::Scalar(name));
                }
            }
            // OpenRC: the runlevel dir exists even without systemd.
            if ctx.is_dir("/run/openrc") || ctx.exists("/etc/runlevels") {
                return Ok(InfoValue::Scalar("openrc".into()));
            }
            Ok(InfoValue::Scalar("unknown".into()))
        }
        #[cfg(target_os = "macos")]
        {
            let _ = ctx;
            Ok(InfoValue::Scalar("launchd".into()))
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            let _ = ctx;
            Ok(InfoValue::Scalar("unknown".into()))
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::fs::{test_ctx, MockFs};

    #[test]
    fn detects_systemd_by_run_dir() {
        let ctx = test_ctx(MockFs::new().dir("/run/systemd/system"));
        let v = InitsystemModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "systemd");
    }

    #[test]
    fn falls_back_to_pid1_comm() {
        let ctx = test_ctx(MockFs::new().file("/proc/1/comm", "openrc-init\n"));
        let v = InitsystemModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "openrc-init");
    }

    #[test]
    fn unknown_when_nothing_found() {
        let ctx = test_ctx(MockFs::new());
        let v = InitsystemModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "unknown");
    }
}
