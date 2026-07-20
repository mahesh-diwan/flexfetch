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
