use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct FsDeepModule;

impl Module for FsDeepModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        // Phase 4.6: filesystem deep dive — pure sysfs/file reads, zero spawns.
        // Every probe degrades gracefully (missing paths → empty map → omitted).
        let mut map = HashMap::new();

        // ZRAM: compression ratio = compr_data_size / mem_used_total.
        if let Ok(entries) = ctx.read_dir("/sys/block") {
            let zrams: Vec<_> = entries
                .iter()
                .filter(|p| {
                    p.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .starts_with("zram")
                })
                .collect();
            if !zrams.is_empty() {
                let total_disk = read_u64(ctx, "/sys/block/zram0/disksize") / 1024; // KB
                let compr = read_u64(ctx, "/sys/block/zram0/compr_data_size") / 1024; // KB
                let used = read_u64(ctx, "/sys/block/zram0/mem_used_total") / 1024; // KB
                if total_disk > 0 && compr > 0 {
                    let ratio = total_disk as f64 / compr as f64;
                    map.insert("zram_ratio".into(), format!("{ratio:.1}x"));
                }
                if used > 0 {
                    map.insert("zram_used".into(), format!("{used} KB"));
                }
            }
        }

        // LUKS / dm-crypt: /sys/class/block/*/dm/uuid starts with CRYPT-LUKS.
        if let Ok(entries) = ctx.read_dir("/sys/class/block") {
            let mut luks: Vec<String> = Vec::new();
            for entry in entries {
                let dm_uuid = entry.join("dm/uuid");
                if let Ok(u) = ctx.read_file(&dm_uuid) {
                    if u.trim_start().starts_with("CRYPT-LUKS") {
                        luks.push(
                            entry
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                        );
                    }
                }
            }
            if !luks.is_empty() {
                map.insert("luks_devices".into(), luks.join(","));
            }
        }

        // LVM: /sys/block/dm-*/dm/name gives the logical volume name.
        if let Ok(entries) = ctx.read_dir("/sys/block") {
            let mut lvm: Vec<String> = Vec::new();
            for entry in entries {
                let name = entry
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if name.starts_with("dm-") {
                    if let Ok(lv) = ctx.read_file(entry.join("dm/name")) {
                        let lv = lv.trim();
                        if !lv.is_empty() {
                            lvm.push(lv.to_string());
                        }
                    }
                }
            }
            if !lvm.is_empty() {
                map.insert("lvm_volumes".into(), lvm.join(","));
            }
        }

        // BTRFS: device count + compression from /proc/mounts + /sys/fs/btrfs.
        if let Ok(content) = ctx.read_file("/proc/mounts") {
            let mut btrfs_mounts = 0usize;
            let mut compress = None;
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 && parts[2] == "btrfs" {
                    btrfs_mounts += 1;
                    if compress.is_none() {
                        for opt in parts[3].split(',') {
                            if let Some(c) = opt.strip_prefix("compress=") {
                                compress = Some(c.to_string());
                            }
                        }
                    }
                }
            }
            if btrfs_mounts > 0 {
                map.insert("btrfs_mounts".into(), btrfs_mounts.to_string());
                if let Some(c) = compress {
                    map.insert("btrfs_compression".into(), c);
                }
            }
        }
        // BTRFS device count: one subdir per filesystem in /sys/fs/btrfs/.
        if let Ok(entries) = ctx.read_dir("/sys/fs/btrfs") {
            if !entries.is_empty() {
                map.insert("btrfs_filesystems".into(), entries.len().to_string());
            }
        }

        Ok(InfoValue::Map(map))
    }
}

fn read_u64(ctx: &Context, path: &str) -> u64 {
    ctx.read_file(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn zram_ratio_computation() {
        // total_disk 4 GiB, compr 1 GiB → 4.0x
        let total_kb = 4.0 * 1024.0 * 1024.0;
        let compr_kb = 1.0 * 1024.0 * 1024.0;
        let ratio = total_kb / compr_kb;
        assert_eq!(format!("{ratio:.1}x"), "4.0x");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn zram_from_mock_sysfs() {
        use super::*;
        use crate::fs::{test_ctx, MockFs};
        // disksize 4 GiB, compr 1 GiB, used 512 MiB.
        let ctx = test_ctx(
            MockFs::new()
                .file("/sys/block/zram0/disksize", "4294967296\n")
                .file("/sys/block/zram0/compr_data_size", "1073741824\n")
                .file("/sys/block/zram0/mem_used_total", "536870912\n"),
        );
        let v = FsDeepModule.collect(&ctx).unwrap();
        let map = match v {
            InfoValue::Map(m) => m,
            _ => panic!("expected map"),
        };
        assert_eq!(map.get("zram_ratio").map(String::as_str), Some("4.0x"));
        // 536870912 bytes / 1024 = 524288 KB
        assert_eq!(map.get("zram_used").map(String::as_str), Some("524288 KB"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn btrfs_from_mock_mounts() {
        use super::*;
        use crate::fs::{test_ctx, MockFs};
        let mounts = "/dev/nvme0n1p2 / btrfs rw,compress=zstd:3 0 0\n\
                      /dev/nvme0n1p2 /home btrfs rw,compress=zstd:3 0 0\n";
        let ctx = test_ctx(MockFs::new().file("/proc/mounts", mounts));
        let v = FsDeepModule.collect(&ctx).unwrap();
        let map = match v {
            InfoValue::Map(m) => m,
            _ => panic!("expected map"),
        };
        assert_eq!(map.get("btrfs_mounts").map(String::as_str), Some("2"));
        assert_eq!(
            map.get("btrfs_compression").map(String::as_str),
            Some("zstd:3")
        );
    }
}
