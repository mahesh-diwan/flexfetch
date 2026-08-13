#![cfg_attr(not(unix), allow(unused_mut))] // collectors mutate only on unix

use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
#[cfg(unix)]
use std::process::Command;

pub struct BluetoothModule;

impl Module for BluetoothModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        // Reuse the shared cache (60 s TTL): the adapter/paired-device state
        // barely changes between runs, so repeated invocations skip the two
        // bluetoothctl spawns (~15 ms total).
        if let Ok(cache) = ctx.cache.lock() {
            if let Some(cached) = cache.get("bluetooth") {
                if let Ok(value) = serde_json::from_str::<InfoValue>(&cached) {
                    return Ok(value);
                }
            }
        }

        let result = collect_uncached(ctx);
        if let Ok(value) = &result {
            if let Ok(mut cache) = ctx.cache.lock() {
                if let Ok(json) = serde_json::to_string(value) {
                    cache.set("bluetooth", json);
                }
            }
        }
        result
    }
}

fn collect_uncached(_ctx: &Context) -> Result<InfoValue> {
    let mut map = HashMap::new();

    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = Command::new("bluetoothctl").arg("show").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let line = line.trim();
                if let Some(v) = line.strip_prefix("Name: ") {
                    map.insert("adapter".into(), v.to_string());
                } else if let Some(v) = line.strip_prefix("Powered: ") {
                    let state = if v == "yes" { "on" } else { "off" };
                    map.insert("state".into(), state.to_string());
                }
            }
        }
        if let Ok(output) = Command::new("bluetoothctl")
            .args(["devices", "Paired"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let count = stdout.lines().count();
            if count > 0 {
                map.insert("devices".into(), format!("{count} paired"));
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("system_profiler")
            .arg("SPBluetoothDataType")
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut in_details = false;
            for line in stdout.lines() {
                let trimmed = line.trim();
                if trimmed.contains("Bluetooth Adapter") || trimmed.contains("LMP Version") {
                    // next non-empty line is often the adapter name
                    in_details = true;
                } else if in_details && !trimmed.is_empty() && !trimmed.starts_with('(') {
                    map.insert("adapter".into(), trimmed.trim_end_matches(':').to_string());
                    in_details = false;
                }
                if trimmed.starts_with("State: ") {
                    map.insert(
                        "state".into(),
                        trimmed.trim_start_matches("State: ").to_string(),
                    );
                }
            }
            // count connected devices from "Connected: Yes" lines
            let connected = stdout
                .lines()
                .filter(|l| l.trim() == "Connected: Yes")
                .count();
            if connected > 0 {
                map.insert("devices".into(), format!("{connected} connected"));
            }
        }
    }

    if map.is_empty() {
        return Ok(InfoValue::Scalar("N/A".into()));
    }
    Ok(InfoValue::Map(map))
}
