use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct NetworkModule;

impl Module for NetworkModule {
    #[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        // Explicit type: on macOS the `&mut nets` read loop precedes the first
        // `push`, so inference can't resolve the element type there.
        let mut nets: Vec<HashMap<String, String>> = Vec::new();

        #[cfg(target_os = "linux")]
        {
            // Phase 4.1: one getifaddrs() call gives every interface's addresses
            // (no `ip` subprocess); the MAC comes straight from sysfs (no `cat`).
            let addrs = iface_addrs();
            if let Ok(entries) = ctx.read_dir("/sys/class/net/") {
                for path in entries {
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    if name == "lo"
                        || name.starts_with("docker")
                        || name.starts_with("br-")
                        || name.starts_with("veth")
                        || name.starts_with("virbr")
                    {
                        continue;
                    }
                    let (ip4, ip6) = addrs.get(&name).cloned().unwrap_or_default();
                    let mac = ctx
                        .read_file(format!("/sys/class/net/{name}/address"))
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
                            // split_whitespace already skips leading whitespace.
                            let ip = line.split_whitespace().nth(1).unwrap_or("").to_string();
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

        // Phase 8.9 — Windows: GetAdaptersInfo (description + MAC + IPv4 list).
        #[cfg(target_os = "windows")]
        {
            use std::ffi::CStr;
            use windows_sys::Win32::NetworkManagement::IpHelper::{
                GetAdaptersInfo, IP_ADAPTER_INFO,
            };

            // Grow-to-fit with a retry loop: the adapter list can change between
            // the probe and the fetch (ERROR_BUFFER_OVERFLOW = 111). The buffer
            // comes from Vec, whose allocator guarantees alignment >= max_align_t
            // (>= the struct's 4-byte alignment).
            let mut size: u32 = 0;
            unsafe { GetAdaptersInfo(std::ptr::null_mut(), &mut size) };
            let mut buf = vec![0u8; size.max(1) as usize];
            loop {
                let r = unsafe { GetAdaptersInfo(buf.as_mut_ptr() as *mut _, &mut size) };
                if r == 111 {
                    buf.resize(size as usize, 0);
                    continue;
                }
                if r != 0 {
                    break; // e.g. no adapters — skip gracefully
                }
                let mut p = buf.as_ptr() as *const IP_ADAPTER_INFO;
                while !p.is_null() {
                    let a = unsafe { &*p };
                    let desc = unsafe { CStr::from_ptr(a.Description.as_ptr()) }
                        .to_string_lossy()
                        .to_string();
                    let mac = (0..a.AddressLength as usize)
                        .map(|i| format!("{:02x}", a.Address[i]))
                        .collect::<Vec<_>>()
                        .join(":");

                    // First non-empty IPv4 in the linked list (IP_ADDR_STRING wraps
                    // an IP_ADDRESS_STRING whose `String` field is a C string in a
                    // fixed [i8; 16]).
                    let mut ip4 = String::new();
                    let mut addr = &a.IpAddressList;
                    loop {
                        let ip = unsafe { CStr::from_ptr(addr.IpAddress.String.as_ptr()) }
                            .to_string_lossy()
                            .to_string();
                        if !ip.is_empty() {
                            ip4 = ip;
                            break;
                        }
                        if addr.Next.is_null() {
                            break;
                        }
                        addr = unsafe { &*addr.Next };
                    }

                    let mut map = HashMap::new();
                    map.insert("name".into(), desc);
                    map.insert("ipv4".into(), ip4);
                    map.insert("ipv6".into(), String::new());
                    map.insert("mac".into(), mac);
                    nets.push(map);

                    p = a.Next;
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
