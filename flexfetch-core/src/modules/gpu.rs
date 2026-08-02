use crate::{Context, InfoValue, Module, Result};

pub struct GpuModule;

impl Module for GpuModule {
    fn name(&self) -> &'static str {
        "gpu"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut gpus = Vec::new();

        #[cfg(target_os = "linux")]
        {
            // Use sysfs first (faster, no process spawn)
            if let Ok(entries) = std::fs::read_dir("/sys/class/drm/") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("card") && name.len() <= 6 {
                        let dev = entry.path().join("device");

                        // Phase 5.8: resolve the vendor:device ID against the
                        // hardware DB (cached + bundled seed) for a friendly
                        // model name; fall back to the driver name otherwise.
                        let vendor = std::fs::read_to_string(dev.join("vendor")).ok();
                        let device = std::fs::read_to_string(dev.join("device")).ok();
                        if let (Some(v), Some(d)) = (vendor, device) {
                            if let Some(friendly) = crate::hardware_db::lookup(&v, &d) {
                                if !gpus.contains(&friendly) {
                                    gpus.push(friendly);
                                }
                                continue;
                            }
                        }

                        let drv = dev.join("driver");
                        if let Ok(link) = std::fs::read_link(&drv) {
                            if let Some(d) = link.file_name() {
                                let d = d.to_string_lossy().to_string();
                                if !gpus.contains(&d) {
                                    gpus.push(d);
                                }
                            }
                        }
                    }
                }
            }

            // Fallback to lspci if sysfs didn't find anything
            if gpus.is_empty() {
                if let Ok(output) = std::process::Command::new("lspci").output() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    for line in stdout.lines() {
                        let lower = line.to_lowercase();
                        if lower.contains("vga")
                            || lower.contains("3d")
                            || lower.contains("display")
                        {
                            if let Some(idx) = line.rfind(':') {
                                let name = line[idx + 1..]
                                    .split('(')
                                    .next()
                                    .unwrap_or("")
                                    .trim()
                                    .to_string();
                                if !name.is_empty() {
                                    gpus.push(name);
                                }
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
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if let Some(val) = line.trim().strip_prefix("Chipset Model:") {
                        gpus.push(val.trim().to_string());
                    }
                }
            }
        }

        Ok(InfoValue::List(gpus))
    }
}
