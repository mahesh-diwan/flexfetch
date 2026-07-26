use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

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
                    let ip4 = std::process::Command::new("ip")
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

                    let ip6 = std::process::Command::new("ip")
                        .args(["-o", "-6", "addr", "show", "dev", &name])
                        .output()
                        .ok()
                        .and_then(|o| {
                            let out = String::from_utf8_lossy(&o.stdout);
                            out.split_whitespace()
                                .nth(3)
                                .map(|s| s.split('/').next().unwrap_or("").to_string())
                        })
                        .unwrap_or_default();

                    let mac = std::process::Command::new("cat")
                        .args([format!("/sys/class/net/{}/address", name)])
                        .output()
                        .ok()
                        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                        .unwrap_or_default();

                    let mut iface = HashMap::new();
                    iface.insert("name".into(), name);
                    iface.insert("ipv4".into(), ip4);
                    iface.insert("ipv6".into(), ip6);
                    iface.insert("mac".into(), mac);
                    nets.push(iface);
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("ifconfig").output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut current_iface = String::new();
                for line in stdout.lines() {
                    if line.is_empty() || line.as_bytes()[0] == b' ' || line.as_bytes()[0] == b'\t'
                    {
                        if line.trim().starts_with("inet ") && !current_iface.is_empty() {
                            let ip = line
                                .trim()
                                .split_whitespace()
                                .nth(1)
                                .unwrap_or("")
                                .to_string();
                            // Find and update the iface
                            for iface in &mut nets {
                                if iface.get("name") == Some(&current_iface) {
                                    iface.insert("ipv4".into(), ip);
                                    break;
                                }
                            }
                        }
                        continue;
                    }
                    if let Some(iface) = line.split(':').next() {
                        if iface == "lo0" || iface.is_empty() {
                            continue;
                        }
                        let state = if line.contains("UP") { "up" } else { "down" };
                        let mut map = HashMap::new();
                        map.insert("name".into(), iface.to_string());
                        map.insert("state".into(), state);
                        map.insert("ipv4".into(), String::new());
                        map.insert("ipv6".into(), String::new());
                        map.insert("mac".into(), String::new());
                        current_iface = iface.to_string();
                        nets.push(map);
                    }
                }
            }
        }

        Ok(InfoValue::Table(nets))
    }
}
