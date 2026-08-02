use crate::{Context, InfoValue, Module, Result};

pub struct DiskModule;

impl Module for DiskModule {
    fn name(&self) -> &'static str {
        "disk"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut disks = Vec::new();

        let mut mounts: Vec<String> = if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
            content
                .lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let (mp, fstype) = (parts[1], parts[2]);
                        if [
                            "ext2",
                            "ext3",
                            "ext4",
                            "btrfs",
                            "xfs",
                            "zfs",
                            "apfs",
                            "f2fs",
                            "overlay",
                            "overlayfs",
                        ]
                        .contains(&fstype)
                            && (mp == "/" || mp == "/home")
                        {
                            return Some(mp.to_string());
                        }
                    }
                    None
                })
                .collect()
        } else {
            vec!["/".to_string()]
        };
        // Fallback: if nothing matched (e.g. container with an unrecognized root
        // fstype), still show "/" rather than an empty list (df used to show it).
        if mounts.is_empty() {
            mounts.push("/".to_string());
        }

        // Phase 4.1: statvfs syscall instead of a `df` subprocess.
        for mp in &mounts {
            if let Some((total, used, pct)) = statvfs_usage(mp) {
                let entry = format!("{mp}: {total} / {used} {pct}%");
                // Deduplicate: if size+usage match an existing entry, skip
                let dup = disks.iter().any(|e: &String| {
                    e.split(": ").nth(1).map(|rest| rest.to_string())
                        == Some(format!("{total} / {used} {pct}%"))
                });
                if !dup {
                    disks.push(entry);
                }
            }
        }

        #[cfg(target_os = "macos")]
        if disks.is_empty() {
            disks.push("/: unavailable".into());
        }

        Ok(InfoValue::List(disks))
    }
}

/// Filesystem usage via libc::statvfs (no `df` spawn). Returns
/// (total, used, percent) as display strings.
fn statvfs_usage(mp: &str) -> Option<(String, String, String)> {
    let c = std::ffi::CString::new(mp).ok()?;
    let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(c.as_ptr(), &mut st) } != 0 {
        return None;
    }
    let frsize = st.f_frsize as u64;
    let total = st.f_blocks as u64 * frsize;
    let free = st.f_bfree as u64 * frsize;
    let avail = st.f_bavail as u64 * frsize;
    let used = total.saturating_sub(free);
    let pct = (total - avail)
        .checked_mul(100)
        .and_then(|n| n.checked_div(total))
        .map(|p| p.min(100))
        .unwrap_or(0);
    Some((human_size(total), human_size(used), format!("{pct}%")))
}

fn human_size(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        format!("{:.1}G", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1}M", bytes as f64 / MIB as f64)
    } else {
        format!("{}K", bytes / 1024)
    }
}
