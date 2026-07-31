use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::process::Command;

pub struct BluetoothModule;

impl Module for BluetoothModule {
    fn name(&self) -> &'static str {
        "bluetooth"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
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
}
