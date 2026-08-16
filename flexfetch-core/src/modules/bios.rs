use crate::{Context, InfoValue, Module, Result};

/// BIOS vendor + version, straight from DMI (`/sys/class/dmi/id`).
pub struct BiosModule;

impl Module for BiosModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            let vendor = ctx
                .read_file("/sys/class/dmi/id/bios_vendor")
                .ok()
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            let version = ctx
                .read_file("/sys/class/dmi/id/bios_version")
                .ok()
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            match (vendor.as_str(), version.as_str()) {
                ("", "") => Ok(InfoValue::Scalar("unknown".into())),
                (v, "") => Ok(InfoValue::Scalar(v.to_string())),
                (_, ver) if vendor.is_empty() => Ok(InfoValue::Scalar(ver.to_string())),
                (v, ver) => Ok(InfoValue::Scalar(format!("{v} {ver}"))),
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
    fn vendor_plus_version() {
        let ctx = test_ctx(
            MockFs::new()
                .file("/sys/class/dmi/id/bios_vendor", "LENOVO\n")
                .file("/sys/class/dmi/id/bios_version", "LTCN31WW\n"),
        );
        let v = BiosModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "LENOVO LTCN31WW");
    }

    #[test]
    fn unknown_when_dmi_missing() {
        let ctx = test_ctx(MockFs::new());
        let v = BiosModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "unknown");
    }
}
