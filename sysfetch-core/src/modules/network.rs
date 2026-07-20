use crate::{Module, InfoValue, Context, Result};

pub struct NetworkModule;

impl Module for NetworkModule {
    fn name(&self) -> &'static str { "network" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut interfaces = Vec::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
                for entry in entries.flatten() {
                    let name = entry.file_name();
                    let name = name.to_string_lossy().to_string();
                    if name == "lo" { continue; }
                    let ip = get_ip_linux(&name);
                    let state_path = entry.path().join("operstate");
                    let status = std::fs::read_to_string(&state_path)
                        .unwrap_or_default().trim().to_string();
                    interfaces.push(format!("{name} {ip} ({status})"));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(out) = std::process::Command::new("ifconfig")
                .args(["-l"])
                .output()
            {
                let s = String::from_utf8_lossy(&out.stdout);
                for name in s.split_whitespace() {
                    if name == "lo0" { continue; }
                    let ip = get_ip_macos(name);
                    interfaces.push(format!("{name} {ip}"));
                }
            }
        }

        Ok(InfoValue::List(interfaces))
    }
}

#[cfg(target_os = "linux")]
fn get_ip_linux(iface: &str) -> String {
    if let Ok(out) = std::process::Command::new("ip")
        .args(["-4", "addr", "show", iface])
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout);
        for line in s.lines() {
            if let Some(addr) = line.trim().strip_prefix("inet ") {
                if let Some(ip) = addr.split('/').next() {
                    return ip.to_string();
                }
            }
        }
    }
    "no ip".into()
}

#[cfg(target_os = "macos")]
fn get_ip_macos(iface: &str) -> String {
    if let Ok(out) = std::process::Command::new("ifconfig")
        .args([iface])
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout);
        for line in s.lines() {
            if let Some(addr) = line.trim().strip_prefix("inet ") {
                if let Some(ip) = addr.split_whitespace().next() {
                    return ip.to_string();
                }
            }
        }
    }
    "no ip".into()
}
