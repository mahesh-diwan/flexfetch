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
                    let pct = parts[4].trim_end_matches('%').parse::<u8>().unwrap_or(0);
                    let filled = (pct / 10).min(10) as usize;
                    let empty = (10 - filled).min(10) as usize;
                    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
                    let entry = format!(
                        "{}: {} / {} {} {}",
                        parts[5], parts[2], parts[1], bar, parts[4]
                    );
                    // Deduplicate: if size+usage match an existing entry, skip
                    let dup = disks.iter().any(|e: &String| {
                        e.split(": ").nth(1).map(|rest| rest.to_string())
                            == Some(format!("{} / {} {} {}", parts[2], parts[1], bar, parts[4]))
                    });
                    if !dup {
                        disks.push(entry);
                    }
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
