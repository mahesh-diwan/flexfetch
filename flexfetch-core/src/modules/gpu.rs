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
            if let Ok(output) = std::process::Command::new("lspci").output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    let lower = line.to_lowercase();
                    if lower.contains("vga") || lower.contains("3d") || lower.contains("display") {
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

            if gpus.is_empty() {
                if let Ok(entries) = std::fs::read_dir("/sys/class/drm/") {
                    for entry in entries.flatten() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if name.starts_with("card") && name.len() <= 6 {
                            let drv = entry.path().join("device").join("driver");
                            if let Ok(link) = std::fs::read_link(&drv) {
                                if let Some(d) = link.file_name() {
                                    gpus.push(d.to_string_lossy().to_string());
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
