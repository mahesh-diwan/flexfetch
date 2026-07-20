use crate::{Module, InfoValue, Context, Result};

pub struct GpuModule;

impl Module for GpuModule {
    fn name(&self) -> &'static str { "gpu" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut devices = Vec::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(out) = std::process::Command::new("lspci")
                .args(["-mm"])
                .output()
            {
                let s = String::from_utf8_lossy(&out.stdout);
                for line in s.lines() {
                    let lower = line.to_lowercase();
                    if lower.contains("vga") || lower.contains("3d") || lower.contains("display") {
                        let parts: Vec<&str> = line.split('"').collect();
                        if parts.len() >= 3 {
                            devices.push(parts[1].trim().to_string());
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(out) = std::process::Command::new("system_profiler")
                .args(["SPDisplaysDataType"])
                .output()
            {
                let s = String::from_utf8_lossy(&out.stdout);
                for line in s.lines() {
                    if let Some(val) = line.strip_prefix("Chipset Model: ") {
                        devices.push(val.trim().to_string());
                    }
                }
            }
        }

        Ok(InfoValue::List(devices))
    }
}
