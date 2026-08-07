use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;

pub struct CpuCacheModule;

impl Module for CpuCacheModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        // `mut` is only needed on Linux (the /sys cache dirs are the only
        // inserts); macOS leaves the map empty, so silence the unused warning.
        #[allow(unused_mut)]
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            if let Ok(entries) = std::fs::read_dir("/sys/devices/system/cpu/cpu0/cache") {
                for entry in entries.flatten() {
                    let dir = entry.path();
                    if !dir.is_dir() {
                        continue;
                    }
                    let level = std::fs::read_to_string(dir.join("level"))
                        .ok()
                        .and_then(|s| s.trim().parse::<u32>().ok());
                    let cache_type = std::fs::read_to_string(dir.join("type"))
                        .ok()
                        .map(|s| s.trim().to_string());
                    let size = std::fs::read_to_string(dir.join("size"))
                        .ok()
                        .map(|s| s.trim().to_string());

                    if let (Some(lvl), Some(sz)) = (level, size) {
                        let key = match (lvl, cache_type.as_deref()) {
                            (1, Some("Data")) => "l1d",
                            (1, Some("Instruction")) => "l1i",
                            (1, _) => "l1",
                            (2, _) => "l2",
                            (3, _) => "l3",
                            _ => continue,
                        };
                        // Keep first occurrence per key (index0 is primary)
                        map.entry(key.into()).or_insert(sz);
                    }
                }
            }
        }

        if map.is_empty() {
            return Ok(InfoValue::Scalar("unknown".into()));
        }
        Ok(InfoValue::Map(map))
    }
}
