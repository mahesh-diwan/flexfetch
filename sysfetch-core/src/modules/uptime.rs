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
