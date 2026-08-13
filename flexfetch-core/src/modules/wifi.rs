use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::process::Command;

pub struct WifiModule;

impl Module for WifiModule {
    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        // Reuse the shared cache (60 s TTL) so repeated invocations skip the
        // ~35 ms nmcli spawn entirely — the SSID/signal barely change between
        // runs (same pattern as the publicip module).
        if let Ok(cache) = ctx.cache.lock() {
            if let Some(cached) = cache.get("wifi") {
                if let Ok(value) = serde_json::from_str::<InfoValue>(&cached) {
                    return Ok(value);
                }
            }
        }

        let result = collect_uncached(ctx);
        if let Ok(value) = &result {
            if let Ok(mut cache) = ctx.cache.lock() {
                if let Ok(json) = serde_json::to_string(value) {
                    cache.set("wifi", json);
                }
            }
        }
        result
    }
}

#[cfg_attr(not(target_os = "linux"), allow(unused_variables))]
fn collect_uncached(ctx: &Context) -> Result<InfoValue> {
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
                            map.insert("signal".into(), format!("{}%", quality_percent(quality)));
                            return Ok(InfoValue::Map(map));
                        }
                    }
                }
                // No iwgetid ssid — try `iw dev <iface> link` (much faster
                // than the nmcli fallback: ~3 ms vs ~35 ms).
                if let Some((ssid, freq)) = iw_link_ssid(&iface) {
                    let mut map = HashMap::new();
                    map.insert("ssid".into(), ssid);
                    map.insert("signal".into(), format!("{}%", quality_percent(quality)));
                    if !freq.is_empty() {
                        map.insert("frequency".into(), freq);
                    }
                    return Ok(InfoValue::Map(map));
                }
                // Active link but no SSID source — fall through to nmcli.
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

/// Read the connected SSID (and frequency) via `iw dev <iface> link` — a
/// ~3 ms spawn vs ~35 ms for nmcli. Returns `(ssid, freq)` where freq is
/// "<mhz> MHz" (empty when the link line is absent).
#[cfg(target_os = "linux")]
fn iw_link_ssid(iface: &str) -> Option<(String, String)> {
    let out = Command::new("iw")
        .args(["dev", iface, "link"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    parse_iw_link(&stdout)
}

/// Parse `iw dev <iface> link` output into (ssid, freq).
#[cfg(target_os = "linux")]
fn parse_iw_link(output: &str) -> Option<(String, String)> {
    let mut ssid = None;
    let mut freq = String::new();
    for line in output.lines() {
        let t = line.trim();
        if let Some(v) = t.strip_prefix("SSID: ") {
            let v = v.trim();
            if !v.is_empty() {
                ssid = Some(v.to_string());
            }
        } else if let Some(v) = t.strip_prefix("freq: ") {
            // "2442.0" → "2442 MHz"
            if let Some(mhz) = v.split('.').next() {
                if !mhz.is_empty() {
                    freq = format!("{mhz} MHz");
                }
            }
        }
    }
    ssid.map(|s| (s, freq))
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
#[cfg(target_os = "linux")]
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
#[cfg(target_os = "linux")]
fn quality_percent(quality: u8) -> u8 {
    (quality.min(70) as u16 * 100 / 70) as u8
}

#[cfg(all(test, target_os = "linux"))]
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

    #[test]
    fn parses_iw_link_connected() {
        let out = "Connected to 26:c9:19:9f:b7:c1 (on wlan0)\n\tSSID: Moto G34 5G\n\tfreq: 2442.0\n\tRX: 100 bytes\n";
        assert_eq!(
            parse_iw_link(out),
            Some(("Moto G34 5G".to_string(), "2442 MHz".to_string()))
        );
    }

    #[test]
    fn parses_iw_link_no_freq() {
        // Some drivers omit the freq line; ssid alone must still parse.
        let out = "Connected to aa:bb:cc:dd:ee:ff (on wlan0)\n\tSSID: Office\n";
        assert_eq!(
            parse_iw_link(out),
            Some(("Office".to_string(), String::new()))
        );
    }

    #[test]
    fn parses_iw_link_not_connected() {
        let out = "Not connected.\n";
        assert_eq!(parse_iw_link(out), None);
    }

    #[test]
    fn parses_iw_link_empty_ssid() {
        let out = "Connected to aa:bb:cc:dd:ee:ff (on wlan0)\n\tSSID: \n";
        assert_eq!(parse_iw_link(out), None);
    }
}
