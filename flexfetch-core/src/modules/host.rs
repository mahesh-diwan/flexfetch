use crate::{Context, InfoValue, Module, Result};

pub struct HostModule;

impl Module for HostModule {
    fn name(&self) -> &'static str {
        "host"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        Ok(InfoValue::Scalar(
            hostname().unwrap_or_else(|| "unknown".into()),
        ))
    }
}

fn hostname() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        std::fs::read_to_string("/proc/sys/kernel/hostname")
            .ok()
            .map(|s| s.trim().to_string())
    }
    #[cfg(target_os = "macos")]
    {
        let mut buf = vec![0u8; 256];
        if unsafe { libc::gethostname(buf.as_mut_ptr() as *mut std::ffi::c_char, 255) } == 0 {
            let len = buf.iter().position(|&c| c == 0).unwrap_or(0);
            Some(
                std::str::from_utf8(&buf[..len])
                    .unwrap_or("mac")
                    .to_string(),
            )
        } else {
            None
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        None
    }
}
