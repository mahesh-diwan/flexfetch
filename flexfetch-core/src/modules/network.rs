use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct NetworkModule;

impl Module for NetworkModule {
    fn name(&self) -> &'static str {
        "network"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        // Explicit type: on macOS the `&mut nets` read loop precedes the first
        // `push`, so inference can't resolve the element type there.
        let mut nets: Vec<HashMap<String, String>> = Vec::new();

        #[cfg(target_os = "linux")]
        {
            // Phase 4.1: one getifaddrs() call gives every interface's addresses
            // (no `ip` subprocess); the MAC comes straight from sysfs (no `cat`).
            let addrs = iface_addrs();
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
                    let (ip4, ip6) = addrs.get(&name).cloned().unwrap_or_default();
                    let mac = std::fs::read_to_string(format!("/sys/class/net/{name}/address"))
                        .ok()
                        .map(|s| s.trim().to_string())
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
                        map.insert("state".into(), state.to_string());
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

/// All interface IPv4/IPv6 addresses via libc::getifaddrs (zero subprocesses).
#[cfg(target_os = "linux")]
fn iface_addrs() -> HashMap<String, (String, String)> {
    let mut out = HashMap::new();
    unsafe {
        let mut ifap: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifap) == 0 {
            let mut cur = ifap;
            while !cur.is_null() {
                let ifa = &*cur;
                if !ifa.ifa_addr.is_null() {
                    let name = std::ffi::CStr::from_ptr(ifa.ifa_name)
                        .to_string_lossy()
                        .to_string();
                    let e = out
                        .entry(name)
                        .or_insert_with(|| (String::new(), String::new()));
                    let family = (*ifa.ifa_addr).sa_family as i32;
                    if family == libc::AF_INET {
                        let sin = &*(ifa.ifa_addr as *const libc::sockaddr_in);
                        let ip = std::net::Ipv4Addr::from(u32::from_be(sin.sin_addr.s_addr));
                        e.0 = ip.to_string();
                    } else if family == libc::AF_INET6 {
                        let sin6 = &*(ifa.ifa_addr as *const libc::sockaddr_in6);
                        let ip = std::net::Ipv6Addr::from(sin6.sin6_addr.s6_addr);
                        e.1 = ip.to_string();
                    }
                }
                cur = ifa.ifa_next;
            }
            libc::freeifaddrs(ifap);
        }
    }
    out
}
