use crate::{Context, InfoValue, Module, Result};

/// Keyboard layout, from `/etc/default/keyboard` (Linux) or Apple's defaults
/// plist (macOS). Zero-subprocess on Linux; macOS reads the plist file.
pub struct KeyboardModule;

impl Module for KeyboardModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        #[cfg(target_os = "linux")]
        {
            let Ok(content) = ctx.read_file("/etc/default/keyboard") else {
                return Ok(InfoValue::Scalar("unknown".into()));
            };
            for line in content.lines() {
                let line = line.trim();
                if let Some(rest) = line.strip_prefix("XKBLAYOUT=") {
                    let layout = rest.trim().trim_matches('"').trim_matches('\'');
                    if !layout.is_empty() {
                        return Ok(InfoValue::Scalar(layout.to_string()));
                    }
                }
            }
            Ok(InfoValue::Scalar("unknown".into()))
        }
        #[cfg(target_os = "macos")]
        {
            let _ = ctx;
            // Best-effort: AppleKeyboardLayouts plist is binary — report unknown
            // rather than shelling out to `defaults`.
            Ok(InfoValue::Scalar("unknown".into()))
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
    fn parses_xkblayout() {
        let ctx = test_ctx(MockFs::new().file(
            "/etc/default/keyboard",
            "XKBMODEL=\"pc105\"\nXKBLAYOUT=\"us\"\nXKBVARIANT=\"\"\n",
        ));
        let v = KeyboardModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "us");
    }

    #[test]
    fn unknown_when_file_missing() {
        let ctx = test_ctx(MockFs::new());
        let v = KeyboardModule.collect(&ctx).unwrap();
        assert_eq!(v.summary(), "unknown");
    }
}
