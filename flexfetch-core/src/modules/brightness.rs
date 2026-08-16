use crate::{Context, InfoValue, Module, Result};

/// Screen brightness as a percentage (backlight sysfs).
pub struct BrightnessModule;

impl Module for BrightnessModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            let Ok(devices) = ctx.read_dir("/sys/class/backlight") else {
                return Ok(InfoValue::Scalar("unknown".into()));
            };
            let mut names: Vec<String> = Vec::new();
            for device in devices {
                let name = device
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let base = device.join("brightness");
                let max_path = device.join("max_brightness");
                let Ok(cur) = ctx.read_file(&base) else {
                    continue;
                };
                let Ok(max) = ctx.read_file(&max_path) else {
                    continue;
                };
                let cur: f64 = cur.trim().parse().unwrap_or(0.0);
                let max: f64 = max.trim().parse().unwrap_or(0.0);
                if max <= 0.0 {
                    continue;
                }
                let pct = ((cur / max) * 100.0).round() as u32;
                if name == "intel_backlight" || name == "amdgpu_bl0" {
                    // Primary panel first
                    names.insert(0, format!("{pct}%"));
                } else {
                    names.push(format!("{pct}%"));
                }
            }
            if names.is_empty() {
                Ok(InfoValue::Scalar("unknown".into()))
            } else {
                Ok(InfoValue::Scalar(names.join(", ")))
            }
        }
        #[cfg(not(target_os = "linux"))]
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
    fn percent_from_backlight_sysfs() {
        let ctx = test_ctx(
            MockFs::new()
                .file("/sys/class/backlight/intel_backlight/brightness", "500\n")
                .file(
                    "/sys/class/backlight/intel_backlight/max_brightness",
                    "2000\n",
                ),
        );
        let v = BrightnessModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "25%");
    }

    #[test]
    fn unknown_when_no_backlight() {
        let ctx = test_ctx(MockFs::new());
        let v = BrightnessModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "unknown");
    }
}
