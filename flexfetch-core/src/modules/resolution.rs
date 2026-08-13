#![cfg_attr(not(unix), allow(unused_mut))] // collectors mutate only on unix

use crate::{Context, InfoValue, Module, Result};

pub struct ResolutionModule;

impl Module for ResolutionModule {
    #[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        // Explicit element type: on Windows no push ever happens (the collectors
        // are Linux-only), so inference cannot otherwise resolve the Vec.
        let mut resolutions: Vec<String> = Vec::new();

        #[cfg(target_os = "linux")]
        {
            // Read from DRM sysfs — works on both X11 and Wayland
            if let Ok(entries) = ctx.read_dir("/sys/class/drm") {
                let mut dirs: Vec<_> = entries;
                dirs.sort_by_key(|e| {
                    e.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                });
                for entry in dirs {
                    let modes_path = entry.join("modes");
                    if ctx.exists(&modes_path) {
                        if let Ok(content) = ctx.read_file(&modes_path) {
                            for line in content.lines() {
                                let m = line.trim();
                                if !m.is_empty() && !resolutions.contains(&m.to_string()) {
                                    resolutions.push(m.to_string());
                                }
                            }
                        }
                    }
                }
            }

            // Fallback to xrandr if available and no DRM modes found
            if resolutions.is_empty() {
                if let Ok(output) = std::process::Command::new("xrandr")
                    .args(["--current"])
                    .output()
                {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines() {
                        if line.contains('*') {
                            if let Some(res) = line.split_whitespace().next() {
                                resolutions.push(res.to_string());
                            }
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("system_profiler")
                .args(["SPDisplaysDataType"])
                .output()
            {
                for line in String::from_utf8_lossy(&output.stdout).lines() {
                    let t = line.trim();
                    if t.starts_with("Resolution:") {
                        if let Some(res) = t.split(':').nth(1) {
                            resolutions.push(res.trim().to_string());
                        }
                    }
                }
            }
        }

        if resolutions.is_empty() {
            return Ok(InfoValue::Scalar("unknown".into()));
        }
        Ok(InfoValue::Scalar(resolutions.join(", ")))
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::fs::{test_ctx, MockFs};

    #[test]
    fn reads_drm_modes_from_mock_sysfs() {
        let ctx = test_ctx(
            MockFs::new()
                .file("/sys/class/drm/card0-DP-1/modes", "1920x1080\n3840x2160\n")
                .file("/sys/class/drm/card0-HDMI-1/modes", "1280x720\n"),
        );
        let v = ResolutionModule.collect(&ctx).unwrap();
        match v {
            InfoValue::Scalar(s) => {
                assert!(s.contains("1920x1080"));
                assert!(s.contains("3840x2160"));
                assert!(s.contains("1280x720"));
            }
            _ => panic!("expected scalar"),
        }
    }

    #[test]
    fn no_drm_yields_unknown_or_fallback() {
        // Empty DRM tree: the module may fall back to a real `xrandr` spawn
        // on a dev machine, so accept either outcome — the point is that an
        // empty sysfs tree never panics or fabricates a mode.
        let ctx = test_ctx(MockFs::new());
        let v = ResolutionModule.collect(&ctx).unwrap();
        match v {
            InfoValue::Scalar(_) => {}
            _ => panic!("expected scalar"),
        }
    }
}
