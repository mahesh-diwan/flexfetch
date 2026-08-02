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
            // Prefer the pure-Rust zbus client when the `music` feature is on;
            // fall back to shelling out to `dbus-send` if it fails or the
            // feature is disabled (keeps the default binary dependency-free).
            #[cfg(feature = "music")]
            {
                if !collect_mpris_zbus(&mut map) {
                    collect_mpris_dbus_send(&mut map);
                }
            }
            #[cfg(not(feature = "music"))]
            {
                collect_mpris_dbus_send(&mut map);
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

/// MPRIS lookup via the pure-Rust zbus client (feature `music`).
/// Returns true if a player was found and its metadata queried (even if the
/// title/artist are empty), false if there is no session bus or no player, in
/// which case the caller falls back to `dbus-send`.
#[cfg(all(target_os = "linux", feature = "music"))]
fn collect_mpris_zbus(map: &mut HashMap<String, String>) -> bool {
    let Ok(conn) = zbus::blocking::Connection::session() else {
        return false;
    };

    // List names on the session bus, find the first MPRIS player.
    let Ok(reply) = conn.call_method(
        Some("org.freedesktop.DBus"),
        "/org/freedesktop/DBus",
        Some("org.freedesktop.DBus"),
        "ListNames",
        &(),
    ) else {
        return false;
    };
    let Ok(names) = reply.body().deserialize::<Vec<String>>() else {
        return false;
    };
    let Some(player) = names.iter().find_map(|n| {
        n.strip_prefix("org.mpris.MediaPlayer2.")
            .map(|p| p.to_string())
    }) else {
        return false;
    };

    map.insert("player".into(), player.clone());
    let dest = format!("org.mpris.MediaPlayer2.{player}");

    // Metadata (a{sv}) — xesam:title + xesam:artist
    let Ok(reply) = conn.call_method(
        Some(dest.as_str()),
        "/org/mpris/MediaPlayer2",
        Some("org.freedesktop.DBus.Properties"),
        "Get",
        &("org.mpris.MediaPlayer2.Player", "Metadata"),
    ) else {
        return true; // player found; metadata just isn't available
    };
    // Bind the body first: the deserialized HashMap borrows from it, and a
    // temporary `reply.body()` would be dropped while `meta` is still in use.
    let body = reply.body();
    let meta: HashMap<String, zbus::zvariant::Value> = match body.deserialize() {
        Ok(m) => m,
        Err(_) => return true,
    };
    if let Some(zbus::zvariant::Value::Str(s)) = meta.get("xesam:title") {
        map.insert("title".into(), s.to_string());
    }
    if let Some(zbus::zvariant::Value::Array(arr)) = meta.get("xesam:artist") {
        // Array::get returns Result<Option<&Value>> (a read can fail on a
        // malformed variant), so match through the Ok layer.
        if let Ok(Some(zbus::zvariant::Value::Str(s))) = arr.get(0) {
            map.insert("artist".into(), s.to_string());
        }
    }

    // Playback status
    let Ok(reply) = conn.call_method(
        Some(dest.as_str()),
        "/org/mpris/MediaPlayer2",
        Some("org.freedesktop.DBus.Properties"),
        "Get",
        &("org.mpris.MediaPlayer2.Player", "PlaybackStatus"),
    ) else {
        return true;
    };
    if let Ok(status) = reply.body().deserialize::<String>() {
        map.insert("status".into(), status);
    }

    true
}

/// MPRIS lookup by shelling out to `dbus-send` (the dependency-free fallback).
#[cfg(target_os = "linux")]
fn collect_mpris_dbus_send(map: &mut HashMap<String, String>) {
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
                parse_mpris_metadata(&meta, map);
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
