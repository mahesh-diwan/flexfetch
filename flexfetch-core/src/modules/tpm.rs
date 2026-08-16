use crate::{Context, InfoValue, Module, Result};

/// TPM (Trusted Platform Module) presence + version from sysfs.
pub struct TpmModule;

impl Module for TpmModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            let Ok(devices) = ctx.read_dir("/sys/class/tpm") else {
                return Ok(InfoValue::Scalar("unknown".into()));
            };
            let mut versions: Vec<String> = Vec::new();
            for device in devices {
                let description = device.join("device/description");
                if let Ok(desc) = ctx.read_file(&description) {
                    let desc = desc.trim().to_string();
                    if !desc.is_empty() {
                        versions.push(desc);
                    }
                }
            }
            if versions.is_empty() {
                Ok(InfoValue::Scalar("unknown".into()))
            } else {
                Ok(InfoValue::Scalar(versions.join(", ")))
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
    fn reads_tpm_description() {
        let ctx = test_ctx(
            MockFs::new().file("/sys/class/tpm/tpm0/device/description", "TPM 2.0 Device\n"),
        );
        let v = TpmModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "TPM 2.0 Device");
    }

    #[test]
    fn unknown_when_no_tpm() {
        let ctx = test_ctx(MockFs::new());
        let v = TpmModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "unknown");
    }
}
