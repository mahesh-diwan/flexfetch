use crate::{Context, InfoValue, Module, Result};

/// Motherboard vendor + name, straight from DMI (`/sys/class/dmi/id`).
pub struct BoardModule;

impl Module for BoardModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            let vendor = ctx
                .read_file("/sys/class/dmi/id/board_vendor")
                .ok()
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            let name = ctx
                .read_file("/sys/class/dmi/id/board_name")
                .ok()
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            match (vendor.as_str(), name.as_str()) {
                ("", "") => Ok(InfoValue::Scalar("unknown".into())),
                (v, "") => Ok(InfoValue::Scalar(v.to_string())),
                (_, n) if vendor.is_empty() => Ok(InfoValue::Scalar(n.to_string())),
                (v, n) => Ok(InfoValue::Scalar(format!("{v} {n}"))),
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
    fn vendor_plus_name() {
        let ctx = test_ctx(
            MockFs::new()
                .file("/sys/class/dmi/id/board_vendor", "LENOVO\n")
                .file("/sys/class/dmi/id/board_name", "LNVNB161216\n"),
        );
        let v = BoardModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "LENOVO LNVNB161216");
    }

    #[test]
    fn unknown_when_dmi_missing() {
        let ctx = test_ctx(MockFs::new());
        let v = BoardModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "unknown");
    }
}
