use crate::{Context, InfoValue, Module, Result};

/// Load average — the 1/5/15-minute averages from the kernel.
///
/// Zero-subprocess: `/proc/loadavg` (Linux) or `getloadavg(3)` (macOS).
pub struct LoadavgModule;

impl Module for LoadavgModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            let Ok(content) = ctx.read_file("/proc/loadavg") else {
                return Ok(InfoValue::Scalar("unknown".into()));
            };
            let parts: Vec<&str> = content.split_whitespace().take(3).collect();
            if parts.len() < 3 {
                return Ok(InfoValue::Scalar("unknown".into()));
            }
            Ok(InfoValue::Scalar(format!(
                "{} {} {}",
                parts[0], parts[1], parts[2]
            )))
        }
        #[cfg(target_os = "macos")]
        {
            let mut load: [f64; 3] = [0.0; 3];
            if unsafe { libc::getloadavg(load.as_mut_ptr(), 3) } == 3 {
                Ok(InfoValue::Scalar(format!(
                    "{:.2} {:.2} {:.2}",
                    load[0], load[1], load[2]
                )))
            } else {
                Ok(InfoValue::Scalar("unknown".into()))
            }
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
    fn parses_mock_loadavg() {
        let ctx = test_ctx(MockFs::new().file("/proc/loadavg", "2.34 3.45 4.03 1/234 5678\n"));
        let v = LoadavgModule.collect(&ctx).unwrap();
        assert_eq!(
            v.summary(),
            "2.34 3.45 4.03",
            "only the three averages should be shown"
        );
    }

    #[test]
    fn unknown_when_file_missing() {
        let ctx = test_ctx(MockFs::new());
        let v = LoadavgModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "unknown");
    }
}
