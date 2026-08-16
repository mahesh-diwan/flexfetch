use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

/// Local IP addresses (non-loopback, non-virtual interfaces).
///
/// Reuses the same single `getifaddrs()` call as the network module — one
/// syscall, zero subprocesses. Unlike `network`, which renders the full
/// per-interface table, this module shows the primary addresses compactly.
pub struct LocalipModule;

impl Module for LocalipModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();
        for (name, (ip4, ip6)) in iface_addrs() {
            if name == "lo" {
                continue;
            }
            map.insert(name, format!("{} {}", ip4, ip6).trim().to_string());
        }
        if map.is_empty() {
            return Ok(InfoValue::Scalar("unknown".into()));
        }
        // Primary v4 first, then everything else — stable order for tests.
        let mut addrs: Vec<String> = Vec::new();
        let mut rest: Vec<String> = Vec::new();
        for (name, addr) in map {
            let entry = if addr.trim().is_empty() {
                format!("{name}: -")
            } else {
                format!("{name}: {addr}")
            };
            if addr.contains('.') {
                addrs.push(entry);
            } else {
                rest.push(entry);
            }
        }
        addrs.sort();
        rest.sort();
        addrs.extend(rest);
        Ok(InfoValue::Scalar(addrs.join(", ")))
    }
}

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

#[cfg(not(target_os = "linux"))]
fn iface_addrs() -> HashMap<String, (String, String)> {
    HashMap::new()
}
