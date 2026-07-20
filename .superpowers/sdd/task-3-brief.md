# Task 3: Simple detection modules (os, host, kernel, uptime, locale)

**Files:**
- Create: `sysfetch-core/src/modules/mod.rs` (MUST include ALL module declarations: os, host, kernel, uptime, locale, cpu, memory, disk, gpu, network, battery, processes, shell, terminal, de, wm, packages, colors, custom)
- Create: `sysfetch-core/src/modules/os.rs`
- Create: `sysfetch-core/src/modules/host.rs`
- Create: `sysfetch-core/src/modules/kernel.rs`
- Create: `sysfetch-core/src/modules/uptime.rs`
- Create: `sysfetch-core/src/modules/locale.rs`
- Modify: `sysfetch-core/src/lib.rs` (add `pub mod modules;`)

## modules/mod.rs

```rust
pub mod os;
pub mod host;
pub mod kernel;
pub mod uptime;
pub mod locale;
pub mod cpu;
pub mod memory;
pub mod disk;
pub mod gpu;
pub mod network;
pub mod battery;
pub mod processes;
pub mod shell;
pub mod terminal;
pub mod de;
pub mod wm;
pub mod packages;
pub mod colors;
pub mod custom;
```

## os.rs

```rust
use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct OsModule;

impl Module for OsModule {
    fn name(&self) -> &'static str { "os" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if let Some((key, val)) = line.split_once('=') {
                        let clean = val.trim_matches('"');
                        match key {
                            "NAME" => { map.insert("name".into(), clean.into()); }
                            "PRETTY_NAME" => { map.insert("pretty_name".into(), clean.into()); }
                            "VERSION_ID" => { map.insert("version".into(), clean.into()); }
                            "ID" => { map.insert("id".into(), clean.into()); }
                            "BUILD_ID" => { map.insert("build_id".into(), clean.into()); }
                            _ => {}
                        }
                    }
                }
            }
            if !map.contains_key("name") {
                if let Ok(_arch) = std::fs::read_to_string("/etc/arch-release") {
                    map.insert("name".into(), "Arch Linux".into());
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            map.insert("name".into(), "macOS".into());
            if let Ok(output) = std::process::Command::new("sw_vers")
                .arg("-productVersion").output()
            {
                let v = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !v.is_empty() { map.insert("version".into(), v); }
            }
        }

        map.insert("arch".into(), std::env::consts::ARCH.to_string());
        Ok(InfoValue::Map(map))
    }
}
```

## host.rs

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct HostModule;

impl Module for HostModule {
    fn name(&self) -> &'static str { "host" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        Ok(InfoValue::Scalar(
            hostname().unwrap_or_else(|| "unknown".into())
        ))
    }
}

fn hostname() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/sys/kernel/hostname")
            .ok().map(|s| s.trim().to_string())
    }
    #[cfg(target_os = "macos")]
    {
        let mut buf = vec![0u8; 256];
        if unsafe { libc::gethostname(buf.as_mut_ptr() as *mut std::ffi::c_char, 255) } == 0 {
            let len = buf.iter().position(|&c| c == 0).unwrap_or(0);
            Some(std::str::from_utf8(&buf[..len]).unwrap_or("mac").to_string())
        } else {
            None
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    { None }
}
```

## kernel.rs

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct KernelModule;

impl Module for KernelModule {
    fn name(&self) -> &'static str { "kernel" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let uname = std::process::Command::new("uname")
            .args(["-srm"])
            .output()
            .ok()
            .and_then(|o| {
                let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                if s.is_empty() { None } else { Some(s) }
            })
            .unwrap_or_else(|| "unknown".to_string());

        Ok(InfoValue::Scalar(uname))
    }
}
```

## uptime.rs

```rust
use crate::{Module, InfoValue, Context, Result};

pub struct UptimeModule;

impl Module for UptimeModule {
    fn name(&self) -> &'static str { "uptime" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let secs = uptime_secs().unwrap_or(0);
        Ok(InfoValue::Scalar(format_uptime(secs)))
    }
}

fn uptime_secs() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        let content = std::fs::read_to_string("/proc/uptime").ok()?;
        content.split_whitespace().next()?
            .split('.').next()?
            .parse::<u64>().ok()
    }
    #[cfg(target_os = "macos")]
    {
        let mut mib: [i32; 2] = [libc::CTL_KERN, libc::KERN_BOOTTIME];
        let mut boottime = std::mem::MaybeUninit::<libc::timeval>::uninit();
        let mut size = std::mem::size_of::<libc::timeval>();
        if unsafe { libc::sysctl(mib.as_mut_ptr(), 2, boottime.as_mut_ptr().cast(), &mut size, std::ptr::null_mut(), 0) } == 0 {
            let bt = unsafe { boottime.assume_init() };
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).ok()?;
            let boot_secs = bt.tv_sec as u64;
            Some(now.as_secs().saturating_sub(boot_secs))
        } else {
            None
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    { None }
}

pub fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;

    match (days, hours, mins) {
        (0, 0, m) => format!("{m} mins"),
        (0, h, m) => format!("{h}h {m}m"),
        (d, h, m) => format!("{d}d {h}h {m}m"),
    }
}
```

## locale.rs

```rust
use crate::{Module, InfoValue, Context, Result};
use std::collections::HashMap;

pub struct LocaleModule;

impl Module for LocaleModule {
    fn name(&self) -> &'static str { "locale" }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let lang = std::env::var("LANG").unwrap_or_default();
        let encoding = std::env::var("LC_CTYPE")
            .or_else(|_| std::env::var("LC_ALL"))
            .unwrap_or_default();

        let mut map = HashMap::new();
        if !lang.is_empty() { map.insert("lang".into(), lang); }
        if !encoding.is_empty() { map.insert("encoding".into(), encoding); }

        if map.is_empty() {
            Ok(InfoValue::Scalar("unknown".into()))
        } else {
            Ok(InfoValue::Map(map))
        }
    }
}
```

## lib.rs update

Add to sysfetch-core/src/lib.rs:
```rust
pub mod modules;
```

## Verify compilation

Run: `cargo build -p sysfetch-core`

## Test

Add at end of lib.rs:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uptime_format() {
        assert_eq!(crate::modules::uptime::format_uptime(3661), "1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(90061), "1d 1h 1m");
        assert_eq!(crate::modules::uptime::format_uptime(120), "2h 0m");
    }
}
```

## Commit

```bash
git add -A && git commit -m "feat: os, host, kernel, uptime, locale modules"
git push
```
