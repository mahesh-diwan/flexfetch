use crate::{Context, InfoValue, Module, Result};

pub struct ResolutionModule;

impl Module for ResolutionModule {
    fn name(&self) -> &'static str {
        "resolution"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut resolutions = Vec::new();

        #[cfg(target_os = "linux")]
        {
            // Read from DRM sysfs — works on both X11 and Wayland
            if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
                let mut dirs: Vec<_> = entries.flatten().collect();
                dirs.sort_by_key(|e| e.file_name());
                for entry in dirs {
                    let modes_path = entry.path().join("modes");
                    if modes_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&modes_path) {
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
                            if let Some(res) = line.trim().split_whitespace().next() {
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
