# Task 4: Hardware modules (cpu, memory, disk, gpu, network, battery, processes)

**Files:**
- Create: `sysfetch-core/src/modules/cpu.rs`
- Create: `sysfetch-core/src/modules/memory.rs`
- Create: `sysfetch-core/src/modules/disk.rs`
- Create: `sysfetch-core/src/modules/gpu.rs`
- Create: `sysfetch-core/src/modules/network.rs`
- Create: `sysfetch-core/src/modules/battery.rs`
- Create: `sysfetch-core/src/modules/processes.rs`

Note: modules/mod.rs already declares these modules. Only create individual files.

## cpu.rs

```rust
use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct CpuModule;

impl Module for CpuModule {
    fn name(&self) -> &'static str { "cpu" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
                let mut cores = 0usize;
                for line in content.lines() {
                    if let Some(val) = line.strip_prefix("model name\t: ") {
                        if !map.contains_key("model") {
                            map.insert("model".into(), val.trim().into());
                        }
                        cores += 1;
                    }
                    if let Some(val) = line.strip_prefix("cpu MHz\t: ") {
                        if let Ok(mhz) = val.trim().parse::<f64>() {
                            map.insert("freq_mhz".into(), format!("{:.0}", mhz));
                        }
                    }
                    if let Some(val) = line.strip_prefix("cache size\t: ") {
                        map.insert("cache".into(), val.trim().into());
                    }
                }
                map.insert("cores".into(), cores.to_string());
            }
        }

        #[cfg(target_os = "macos")]
        {
            map.insert("model".into(), sysctl_str("machdep.cpu.brand_string").unwrap_or_else(|| "Apple".into()));
            let cores = sysctl_int("hw.ncpu").unwrap_or(0);
            map.insert("cores".into(), cores.to_string());
        }

        Ok(InfoValue::Map(map))
    }
}

#[cfg(target_os = "macos")]
fn sysctl_str(name: &str) -> Option<String> {
    use std::ffi::CString;
    let cname = CString::new(name).ok()?;
    let mut size = 0usize;
    if unsafe { libc::sysctlbyname(cname.as_ptr(), std::ptr::null_mut(), &mut size, std::ptr::null_mut(), 0) } != 0 {
        return None;
    }
    let mut buf = vec![0u8; size];
    if unsafe { libc::sysctlbyname(cname.as_ptr(), buf.as_mut_ptr().cast(), &mut size, std::ptr::null_mut(), 0) } != 0 {
        return None;
    }
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    Some(std::str::from_utf8(&buf[..len]).unwrap_or("").to_string())
}

#[cfg(target_os = "macos")]
fn sysctl_int(name: &str) -> Option<u64> {
    use std::ffi::CString;
    let cname = CString::new(name).ok()?;
    let mut val: u64 = 0;
    let mut size = std::mem::size_of::<u64>();
    if unsafe { libc::sysctlbyname(cname.as_ptr(), &mut val as *mut _ as *mut std::ffi::c_void, &mut size, std::ptr::null_mut(), 0) } == 0 {
        Some(val)
    } else {
        None
    }
}
```

## memory.rs

```rust
use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct MemoryModule;

impl Module for MemoryModule {
    fn name(&self) -> &'static str { "memory" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
                let mut total_kb = 0u64;
                let mut avail_kb = 0u64;
                for line in content.lines() {
                    if let Some(val) = line.strip_prefix("MemTotal:") {
                        total_kb = parse_kb(val);
                    }
                    if let Some(val) = line.strip_prefix("MemAvailable:") {
                        avail_kb = parse_kb(val);
                    }
                }
                let used_kb = total_kb.saturating_sub(avail_kb);
                map.insert("total".into(), fmt_kb(total_kb));
                map.insert("used".into(), fmt_kb(used_kb));
                map.insert("available".into(), fmt_kb(avail_kb));
                if total_kb > 0 {
                    map.insert("percent".into(), format!("{:.0}%", used_kb as f64 / total_kb as f64 * 100.0));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            let total_kb = sysctl_u64("hw.memsize").unwrap_or(0) / 1024;
            if let Ok(out) = std::process::Command::new("vm_stat").output() {
                let s = String::from_utf8_lossy(&out.stdout);
                let page_size = sysctl_u64("hw.pagesize").unwrap_or(4096);
                let mut active = 0u64; let mut wired = 0u64;
                for line in s.lines() {
                    if let Some(v) = line.strip_prefix("Pages active:") {
                        active = v.trim().trim_end_matches('.').parse().unwrap_or(0);
                    }
                    if let Some(v) = line.strip_prefix("Pages wired down:") {
                        wired = v.trim().trim_end_matches('.').parse().unwrap_or(0);
                    }
                }
                let used_kb = ((active + wired) * page_size) / 1024;
                map.insert("total".into(), fmt_kb(total_kb));
                map.insert("used".into(), fmt_kb(used_kb));
                let avail = total_kb.saturating_sub(used_kb);
                map.insert("available".into(), fmt_kb(avail));
                if total_kb > 0 {
                    map.insert("percent".into(), format!("{:.0}%", used_kb as f64 / total_kb as f64 * 100.0));
                }
            } else {
                map.insert("total".into(), fmt_kb(total_kb));
            }
        }

        Ok(InfoValue::Map(map))
    }
}

#[cfg(target_os = "linux")]
fn parse_kb(s: &str) -> u64 {
    s.split_whitespace().next().and_then(|n| n.parse::<u64>().ok()).unwrap_or(0)
}

fn fmt_kb(kb: u64) -> String {
    if kb >= 1024 * 1024 {
        format!("{:.2} GiB", kb as f64 / (1024.0 * 1024.0))
    } else if kb >= 1024 {
        format!("{:.2} MiB", kb as f64 / 1024.0)
    } else {
        format!("{kb} KiB")
    }
}

#[cfg(target_os = "macos")]
fn sysctl_u64(name: &str) -> Option<u64> {
    use std::ffi::CString;
    let cname = CString::new(name).ok()?;
    let mut val: u64 = 0;
    let mut size = std::mem::size_of::<u64>();
    if unsafe { libc::sysctlbyname(cname.as_ptr(), &mut val as *mut _ as *mut std::ffi::c_void, &mut size, std::ptr::null_mut(), 0) } == 0 {
        Some(val)
    } else {
        None
    }
}
```

## disk.rs

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct DiskModule;

impl Module for DiskModule {
    fn name(&self) -> &'static str { "disk" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mounts = get_mounts().unwrap_or_default();
        Ok(InfoValue::List(mounts))
    }
}

fn get_mounts() -> Option<Vec<String>> {
    let mut result = Vec::new();

    #[cfg(target_os = "linux")]
    {
        let content = std::fs::read_to_string("/proc/mounts").ok()?;
        let mut seen = std::collections::HashSet::new();
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 { continue; }
            let mount_point = parts[1];
            if mount_point.starts_with("/sys") || mount_point.starts_with("/proc")
                || mount_point.starts_with("/dev") || mount_point.starts_with("/run")
                || mount_point == "/" { continue; }
            if mount_point.starts_with("/") && seen.insert(mount_point.to_string()) {
                if let Some(usage) = mount_usage(mount_point) {
                    result.push(format!("{mount_point} {usage}"));
                }
            }
        }
        if let Some(usage) = mount_usage("/") {
            result.insert(0, format!("/ {usage}"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(out) = std::process::Command::new("df").args(["-H"]).output() {
            let s = String::from_utf8_lossy(&out.stdout);
            for line in s.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let mount = parts[5];
                    if mount.starts_with("/") {
                        result.push(format!("{mount} {used}/{size}", used=parts[2], size=parts[1]));
                    }
                }
            }
        }
    }

    if result.is_empty() { None } else { Some(result) }
}

fn mount_usage(path: &str) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
        let cpath = std::ffi::CString::new(path).ok()?;
        if unsafe { libc::statvfs(cpath.as_ptr(), &mut stat) } != 0 {
            return None;
        }
        let total = stat.f_blocks as u64 * stat.f_frsize as u64;
        let used = total.saturating_sub(stat.f_bfree as u64 * stat.f_frsize as u64);
        let pct = if total > 0 { format!("{:.0}%", used as f64 / total as f64 * 100.0) } else { "?".into() };
        Some(format!("{}/{} ({})", bytes_fmt(used), bytes_fmt(total), pct))
    }
    #[cfg(not(target_os = "linux"))]
    { None }
}

fn bytes_fmt(bytes: u64) -> String {
    if bytes >= 1 << 40 {
        format!("{:.1}TiB", bytes as f64 / (1u64 << 40) as f64)
    } else if bytes >= 1 << 30 {
        format!("{:.1}GiB", bytes as f64 / (1u64 << 30) as f64)
    } else if bytes >= 1 << 20 {
        format!("{:.1}MiB", bytes as f64 / (1u64 << 20) as f64)
    } else {
        format!("{bytes}B")
    }
}
```

## gpu.rs

```rust
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
```

## network.rs

```rust
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
```

## battery.rs

```rust
use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct BatteryModule;

impl Module for BatteryModule {
    fn name(&self) -> &'static str { "battery" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = std::fs::read_dir("/sys/class/power_supply") {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let type_path = path.join("type");
                    let typ = std::fs::read_to_string(&type_path).unwrap_or_default();
                    if typ.trim() != "Battery" { continue; }
                    let capacity = std::fs::read_to_string(path.join("capacity")).ok()
                        .and_then(|s| s.trim().parse::<u8>().ok()).unwrap_or(0);
                    let status = std::fs::read_to_string(path.join("status"))
                        .unwrap_or_default().trim().to_string();
                    map.insert("percent".into(), format!("{capacity}%"));
                    map.insert("status".into(), status);
                    break;
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(out) = std::process::Command::new("pmset")
                .args(["-g", "batt"])
                .output()
            {
                let s = String::from_utf8_lossy(&out.stdout);
                for line in s.lines() {
                    if let Some(info) = line.trim().strip_prefix("-InternalBattery-0") {
                        if let Some(pct) = info.split_whitespace()
                            .find(|w| w.ends_with('%'))
                        {
                            map.insert("percent".into(), pct.to_string());
                        }
                        if info.contains("discharging") {
                            map.insert("status".into(), "Discharging".into());
                        } else if info.contains("charging") || info.contains("AC") {
                            map.insert("status".into(), "Charging".into());
                        } else if info.contains("charged") {
                            map.insert("status".into(), "Full".into());
                        }
                        break;
                    }
                }
            }
        }

        if map.is_empty() {
            return Ok(InfoValue::Scalar("no battery".into()));
        }
        Ok(InfoValue::Map(map))
    }
}
```

## processes.rs

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct ProcessesModule;

impl Module for ProcessesModule {
    fn name(&self) -> &'static str { "processes" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let count = get_process_count().unwrap_or(0);
        Ok(InfoValue::Scalar(count.to_string()))
    }
}

fn get_process_count() -> Option<usize> {
    #[cfg(target_os = "linux")]
    {
        let entries = std::fs::read_dir("/proc").ok()?;
        let mut count = 0usize;
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.chars().all(|c| c.is_ascii_digit()) {
                count += 1;
            }
        }
        Some(count)
    }
    #[cfg(target_os = "macos")]
    {
        let out = std::process::Command::new("ps")
            .args(["-e", "--no-headers"])
            .output().ok()?;
        let count = String::from_utf8_lossy(&out.stdout).lines().count();
        Some(count)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    { None }
}
```

## Verify compilation

Run: `cargo build -p sysfetch-core`

## Commit

```bash
git add -A && git commit -m "feat: cpu, memory, disk, gpu, network, battery, processes modules"
git push
```
