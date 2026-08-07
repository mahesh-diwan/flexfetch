use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::process::Command;

pub struct WifiModule;

impl Module for WifiModule {
    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        // Phase 4.1: `--rescan no` (space-separated — nmcli rejects `=no` with
        // "invalid extra argument") uses NetworkManager's cached scan instead of
        // triggering a fresh multi-second wifi scan on every fetch — `nmcli dev
        // wifi` without it is the single slowest module (4 s+).
        let out = Command::new("nmcli")
            .args([
                "-t",
                "-f",
                "active,ssid,freq,signal,security",
                "dev",
                "wifi",
                "list",
                "--rescan",
                "no",
            ])
            .output();
        match out {
            Ok(o) if o.status.success() => {
                let stdout = String::from_utf8_lossy(&o.stdout);
                for line in stdout.lines() {
                    // Active connection is first field = "yes"
                    if !line.starts_with("yes:") {
                        continue;
                    }
                    let parts: Vec<&str> = line.split(':').collect();
                    // active:ssid:freq:signal:security (security may contain colons)
                    if parts.len() < 4 {
                        continue;
                    }
                    let ssid = parts[1];
                    let freq = parts[2];
                    let signal = parts[3];
                    let security = if parts.len() > 4 {
                        parts[4..].join(":")
                    } else {
                        String::new()
                    };

                    if ssid.is_empty() {
                        continue;
                    }

                    let mut map = HashMap::new();
                    map.insert("ssid".into(), ssid.to_string());
                    map.insert("signal".into(), format!("{signal}%"));
                    map.insert("frequency".into(), freq.to_string());
                    map.insert("security".into(), security);
                    return Ok(InfoValue::Map(map));
                }
                Ok(InfoValue::Scalar("not connected".into()))
            }
            _ => Ok(InfoValue::Scalar("unknown".into())),
        }
    }
}
