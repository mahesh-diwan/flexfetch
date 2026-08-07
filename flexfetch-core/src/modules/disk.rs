use crate::{Context, InfoValue, Module, Result};

pub struct DiskModule;

impl Module for DiskModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut disks = Vec::new();

        // POSIX path: /proc/mounts roots + statvfs (Phase 4.1, zero subprocess).
        #[cfg(not(target_os = "windows"))]
        {
            let mut mounts: Vec<String> =
                if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
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
            // Fallback: if nothing matched (e.g. container with an unrecognized
            // root fstype), still show "/" rather than an empty list.
            if mounts.is_empty() {
                mounts.push("/".to_string());
            }

            // Phase 4.1: statvfs syscall instead of a `df` subprocess.
            for mp in &mounts {
                if let Some((total, used, pct)) = statvfs_usage(mp) {
                    // `pct` already carries the "%" suffix (statvfs_usage formats
                    // it) — appending another one produced the "82%%" bug.
                    let entry = format!("{mp}: {total} / {used} {pct}");
                    // Deduplicate: if size+usage match an existing entry, skip.
                    let dup = disks.iter().any(|e: &String| {
                        e.split(": ").nth(1).map(|rest| rest.to_string())
                            == Some(format!("{total} / {used} {pct}"))
                    });
                    if !dup {
                        disks.push(entry);
                    }
                }
            }
        }

        // Phase 8.9 — Windows: enumerate fixed drives (A:..Z: bitmask) and read
        // free/total via GetDiskFreeSpaceExW. No subprocesses.
        #[cfg(target_os = "windows")]
        {
            use windows_sys::Win32::Storage::FileSystem::{
                GetDiskFreeSpaceExW, GetDriveTypeW, GetLogicalDrives,
            };

            // DRIVE_FIXED = 3 (GetDriveTypeW return value; windows-sys does not
            // generate the DRIVE_* constants, so the documented SDK value is
            // spelled out here).
            const DRIVE_FIXED: u32 = 3;

            let drives = unsafe { GetLogicalDrives() };
            for i in 0..26 {
                if drives & (1u32 << i) == 0 {
                    continue;
                }
                let letter = (b'A' + i as u8) as char;
                let root = format!("{letter}:\\");
                let root_w = crate::win::wide(&root);
                if unsafe { GetDriveTypeW(root_w.as_ptr()) } != DRIVE_FIXED {
                    continue;
                }
                let mut free_avail: u64 = 0;
                let mut total: u64 = 0;
                let mut free_total: u64 = 0;
                let ok = unsafe {
                    GetDiskFreeSpaceExW(
                        root_w.as_ptr(),
                        &mut free_avail,
                        &mut total,
                        &mut free_total,
                    )
                };
                if ok != 0 && total > 0 {
                    // `used` uses the total-free bytes; the percentage uses
                    // free-avail (same split as Linux's statvfs f_bavail math,
                    // where the two differ only under quotas).
                    let used = total.saturating_sub(free_total);
                    let pct = (total.saturating_sub(free_avail) * 100 / total).min(100);
                    disks.push(format!(
                        "{letter}: {} / {} {pct}%",
                        human_size(total),
                        human_size(used)
                    ));
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
/// (total, used, percent) as display strings. POSIX only (Windows uses
/// GetDiskFreeSpaceExW).
#[cfg(not(target_os = "windows"))]
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
