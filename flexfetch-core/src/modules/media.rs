use crate::{Context, InfoValue, Module, Result};
use std::collections::HashMap;
use std::process::Command;

pub struct MediaModule;

impl Module for MediaModule {
    fn name(&self) -> &'static str {
        "media"
    }

    fn collect(&self, _ctx: &Context) -> Result<InfoValue> {
        let mut map = HashMap::new();

        #[cfg(target_os = "linux")]
        {
            // Find active MPRIS player via dbus
            if let Ok(output) = Command::new("dbus-send")
                .args([
                    "--session",
                    "--dest=org.freedesktop.DBus",
                    "--print-reply",
                    "/org/freedesktop/DBus",
                    "org.freedesktop.DBus.ListNames",
                ])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let mut player = None;
                for line in stdout.lines() {
                    if let Some(name) = line
                        .trim()
                        .strip_prefix("string \"org.mpris.MediaPlayer2.")
                        .and_then(|s| s.strip_suffix('"'))
                    {
                        player = Some(name.to_string());
                        break;
                    }
                }

                if let Some(name) = player {
                    map.insert("player".into(), name.clone());
                    let dest = format!("org.mpris.MediaPlayer2.{name}");
                    // Get metadata
                    if let Ok(out) = Command::new("dbus-send")
                        .args([
                            "--session",
                            &format!("--dest={dest}"),
                            "--print-reply",
                            "/org/mpris/MediaPlayer2",
                            "org.freedesktop.DBus.Properties.Get",
                            "string:org.mpris.MediaPlayer2.Player",
                            "string:Metadata",
                        ])
                        .output()
                    {
                        let meta = String::from_utf8_lossy(&out.stdout);
                        parse_mpris_metadata(&meta, &mut map);
                    }
                    // Get playback status
                    if let Ok(out) = Command::new("dbus-send")
                        .args([
                            "--session",
                            &format!("--dest={dest}"),
                            "--print-reply",
                            "/org/mpris/MediaPlayer2",
                            "org.freedesktop.DBus.Properties.Get",
                            "string:org.mpris.MediaPlayer2.Player",
                            "string:PlaybackStatus",
                        ])
                        .output()
                    {
                        let status = String::from_utf8_lossy(&out.stdout);
                        for line in status.lines() {
                            let t = line.trim();
                            if t.starts_with("string \"") {
                                let val = t.trim_start_matches("string \"").trim_end_matches('"');
                                map.insert("status".into(), val.to_string());
                                break;
                            }
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            // macOS Now Playing — use `nowplaying-cli` if available
            if let Ok(output) = Command::new("nowplaying-cli").arg("get").output() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    if let Some((k, v)) = line.split_once(':') {
                        let v = v.trim();
                        match k.trim() {
                            "Title" => {
                                map.insert("title".into(), v.to_string());
                            }
                            "Artist" => {
                                map.insert("artist".into(), v.to_string());
                            }
                            "Playback Rate" => {
                                let status = if v == "0" { "Paused" } else { "Playing" };
                                map.insert("status".into(), status.to_string());
                            }
                            _ => {}
                        }
                    }
                }
                if !map.is_empty() {
                    map.insert("player".into(), "NowPlaying".into());
                }
            }
        }

        if map.is_empty() {
            return Ok(InfoValue::Scalar("no media".into()));
        }
        Ok(InfoValue::Map(map))
    }
}

#[cfg(target_os = "linux")]
fn parse_mpris_metadata(dbus_output: &str, map: &mut HashMap<String, String>) {
    for line in dbus_output.lines() {
        let t = line.trim();
        if t.contains("string \"xesam:title\"") {
            if let Some(v) = t.split("variant").nth(1) {
                let v = v.trim();
                if v.starts_with("string \"") {
                    map.insert(
                        "title".into(),
                        v.trim_start_matches("string \"")
                            .trim_end_matches('"')
                            .to_string(),
                    );
                }
            }
        } else if t.contains("string \"xesam:artist\"") {
            if let Some(v) = t.split("variant").nth(1) {
                let v = v.trim();
                if v.starts_with("string \"") {
                    map.insert(
                        "artist".into(),
                        v.trim_start_matches("string \"")
                            .trim_end_matches('"')
                            .to_string(),
                    );
                }
            }
        }
    }
}
