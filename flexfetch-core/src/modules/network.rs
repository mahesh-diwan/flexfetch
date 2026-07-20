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
                    if name == "lo"
                        || name.starts_with("docker")
                        || name.starts_with("br-")
                        || name.starts_with("veth")
                        || name.starts_with("virbr")
                    {
                        continue;
                    }
                    if !nets.is_empty() {
                        continue;
                    }
                    let ip = std::process::Command::new("ip")
                        .args(["-o", "-4", "addr", "show", "dev", &name])
                        .output()
                        .ok()
                        .and_then(|o| {
                            let out = String::from_utf8_lossy(&o.stdout);
                            out.split_whitespace()
                                .nth(3)
                                .map(|s| s.split('/').next().unwrap_or("").to_string())
                        })
                        .unwrap_or_default();
                    nets.push(format!("{name}: {ip}"));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("ifconfig").output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if !nets.is_empty() {
                        break;
                    }
                    if line.is_empty() || line.as_bytes()[0] == b' ' || line.as_bytes()[0] == b'\t'
                    {
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
