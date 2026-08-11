use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::process::Command;

pub struct WifiModule;

impl Module for WifiModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        // Tier 1: native /proc read + iwgetid (fast path).
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = ctx.read_file("/proc/net/wireless") {
                if let Some((iface, quality)) = parse_wireless(&content) {
                    if let Ok(o) = Command::new("iwgetid").args(["-r", &iface]).output() {
                        if o.status.success() {
                            let ssid = String::from_utf8_lossy(&o.stdout).trim().to_string();
                            if !ssid.is_empty() {
                                let mut map = HashMap::new();
                                map.insert("ssid".into(), ssid);
                                map.insert(
                                    "signal".into(),
                                    format!("{}%", quality_percent(quality)),
                                );
                                return Ok(InfoValue::Map(map));
                            }
                        }
                    }
                    // Active link but no iwgetid ssid — fall through to nmcli.
                } else {
                    // /proc readable but no active link: kernel wifi is known —
                    // report it without paying for an nmcli spawn.
                    return Ok(InfoValue::Scalar("not connected".into()));
                }
            }
        }

        // Tier 2: fallback to NetworkManager.
        Ok(nmcli_fallback())
    }
}

fn nmcli_fallback() -> InfoValue {
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
                return InfoValue::Map(map);
            }
            InfoValue::Scalar("not connected".into())
        }
        _ => InfoValue::Scalar("unknown".into()),
    }
}

/// Parse `/proc/net/wireless` and return (iface, link quality) of the active
/// interface with the highest quality. Returns `None` when no interface is
/// associated (quality > 0).
fn parse_wireless(content: &str) -> Option<(String, u8)> {
    let mut best: Option<(String, u8)> = None;
    for line in content.lines() {
        let mut tokens = line.split_whitespace();
        let (Some(head), Some(_status), Some(quality)) =
            (tokens.next(), tokens.next(), tokens.next())
        else {
            continue;
        };
        // Data lines start with "iface:"; header lines don't contain a colon.
        if !head.ends_with(':') {
            continue;
        }
        let Ok(quality) = quality.trim_end_matches('.').parse::<u8>() else {
            continue;
        };
        if quality == 0 {
            continue;
        }
        if best.as_ref().is_none_or(|(_, q)| quality > *q) {
            best = Some((head.trim_end_matches(':').to_string(), quality));
        }
    }
    best
}

/// Kernel link quality is 0–70, scaled to 0–100%.
fn quality_percent(quality: u8) -> u8 {
    (quality.min(70) as u16 * 100 / 70) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_highest_quality_iface() {
        let content = "Inter-| sta tion |           current                accumulated
         V1 | V2 | V3 |   V4 |   V5 |     V6 |      V7 |     V8 |     V9 |     V10 |
  wlan1: 0000   30.  -82.        0       0   0.0    0.0000
  wlan0: 0000   70.  -72.        0       0   0.0    0.0000
  wlan2: 0000   50.  -75.        0       0   0.0    0.0000";
        assert_eq!(parse_wireless(content), Some(("wlan0".to_string(), 70)));
    }

    #[test]
    fn single_active_iface() {
        let content = "  wlan0: 0000   55  -60.        0       0   0.0    0.0000";
        assert_eq!(parse_wireless(content), Some(("wlan0".to_string(), 55)));
    }

    #[test]
    fn no_active_iface_returns_none() {
        let headers = "Inter-| sta tion |           current                accumulated
         V1 | V2 | V3 |   V4 |   V5 |     V6 |      V7 |     V8 |     V9 |     V10 |";
        assert_eq!(parse_wireless(headers), None);

        let inactive = "  wlan0: 0000   0.  -100.        0       0   0.0    0.0000";
        assert_eq!(parse_wireless(inactive), None);
    }

    #[test]
    fn quality_percent_mapping() {
        assert_eq!(quality_percent(70), 100);
        assert_eq!(quality_percent(35), 50);
        assert_eq!(quality_percent(7), 10);
        assert_eq!(quality_percent(0), 0);
    }
}
