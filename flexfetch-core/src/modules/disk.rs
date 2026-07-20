use crate::{Context, InfoValue, Module, Result};

pub struct DiskModule;

impl Module for DiskModule {
    fn name(&self) -> &'static str {
        "disk"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut disks = Vec::new();

        let mounts: Vec<String> = if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
            content
                .lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let (mp, fstype) = (parts[1], parts[2]);
                        if ["ext4", "btrfs", "xfs", "zfs", "apfs"].contains(&fstype)
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

        if let Ok(output) = std::process::Command::new("df")
            .args(["-h"])
            .args(&mounts)
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    disks.push(format!(
                        "{}: {} / {} ({})",
                        parts[5], parts[2], parts[1], parts[4]
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
