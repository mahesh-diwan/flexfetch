use crate::{Context, InfoValue, Module, Result};

pub struct NetworkModule;

impl Module for NetworkModule {
    fn name(&self) -> &'static str {
        "network"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut nets = Vec::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = std::fs::read_dir("/sys/class/net/") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name == "lo" {
                        continue;
                    }
                    let state = std::fs::read_to_string(entry.path().join("operstate"))
                        .unwrap_or_default()
                        .trim()
                        .to_string();
                    let speed = std::fs::read_to_string(entry.path().join("speed"))
                        .ok()
                        .and_then(|s| s.trim().parse::<u64>().ok())
                        .unwrap_or(0);
                    if speed > 0 {
                        nets.push(format!("{name}: {state} ({speed} Mbps)"));
                    } else {
                        nets.push(format!("{name}: {state}"));
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("ifconfig").output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if line.starts_with('\t') || line.starts_with(' ') {
                        continue;
                    }
                    if let Some(iface) = line.split(':').next() {
                        if iface == "lo0" || iface.is_empty() {
                            continue;
                        }
                        let state = if line.contains("UP") { "up" } else { "down" };
                        nets.push(format!("{iface}: {state}"));
                    }
                }
            }
        }

        Ok(InfoValue::List(nets))
    }
}
