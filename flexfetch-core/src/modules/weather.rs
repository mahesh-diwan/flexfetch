use crate::{Context, InfoValue, Module, Result};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct WeatherModule;

impl Module for WeatherModule {
    fn name(&self) -> &'static str {
        "weather"
    }

    fn collect(&self, ctx: &Context) -> Result<InfoValue> {
        // Phase 4.14: hand-rolled HTTP/1.1 over TcpStream — no reqwest/hyper.
        // wttr.in serves plain HTTP on :80 (verified; MET Norway is HTTPS-only
        // and 301-redirects, which a bare TCP GET can't follow) and does IP-based
        // geolocation itself, so one request yields current conditions + city.
        // Results cached 10 min.
        let cache_path = ctx.cache_dir.join("weather.json");
        if let Some(cached) = read_cache(&cache_path) {
            return Ok(cached);
        }

        let value = match fetch_current() {
            Some(map) if !map.is_empty() => InfoValue::Map(map),
            _ => InfoValue::Scalar("unavailable".into()),
        };
        if let InfoValue::Map(_) = &value {
            write_cache(&cache_path, &value);
        }
        Ok(value)
    }
}

/// Minimal HTTP/1.1 GET over a plain TCP stream. Returns the response body.
fn http_get(host: &str, path: &str, user_agent: &str) -> Option<String> {
    let mut stream = TcpStream::connect((host, 80)).ok()?;
    stream.set_read_timeout(Some(Duration::from_secs(5))).ok()?;
    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: {user_agent}\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(req.as_bytes()).ok()?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).ok()?;
    let text = String::from_utf8_lossy(&buf);
    // Body follows the blank line after the headers.
    text.split("\r\n\r\n").nth(1).map(|s| s.to_string())
}

/// Fetch current conditions from wttr.in's JSON endpoint (IP-based location).
fn fetch_current() -> Option<std::collections::HashMap<String, String>> {
    let body = http_get("wttr.in", "/?format=j1", "flexfetch/0.16")?;
    let v: serde_json::Value = serde_json::from_str(&body).ok()?;
    let cc = v.get("current_condition")?.as_array()?.first()?;
    let mut out = std::collections::HashMap::new();

    if let Some(t) = cc.get("temp_C").and_then(|x| x.as_str()) {
        out.insert("temperature".into(), format!("{t}°C"));
    }
    if let Some(h) = cc.get("humidity").and_then(|x| x.as_str()) {
        out.insert("humidity".into(), format!("{h}%"));
    }
    if let Some(w) = cc.get("windspeedKmph").and_then(|x| x.as_str()) {
        out.insert("wind".into(), format!("{w} km/h"));
    }
    if let Some(desc) = cc.pointer("/weatherDesc/0/value").and_then(|x| x.as_str()) {
        out.insert("symbol".into(), symbol_emoji(desc));
        out.insert("condition".into(), desc.to_string());
    }
    // City from nearest_area (wttr.in resolves location from the IP).
    if let Some(city) = v
        .pointer("/nearest_area/0/areaName/0/value")
        .and_then(|x| x.as_str())
    {
        out.insert("city".into(), city.to_string());
    }
    Some(out)
}

fn symbol_emoji(condition: &str) -> String {
    let c = condition.to_lowercase();
    if c.contains("sun") {
        "☀️".into()
    } else if c.contains("clear") {
        "🌙".into()
    } else if c.contains("partly") {
        "🌤".into()
    } else if c.contains("cloud") {
        "☁️".into()
    } else if c.contains("rain") {
        "🌧".into()
    } else if c.contains("sleet") {
        "🌨".into()
    } else if c.contains("snow") {
        "❄️".into()
    } else if c.contains("thunder") {
        "⛈".into()
    } else if c.contains("fog") || c.contains("mist") {
        "🌫".into()
    } else if c.contains("overcast") {
        "☁️".into()
    } else {
        "🌡".into()
    }
}

fn read_cache(path: &std::path::Path) -> Option<InfoValue> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let meta = std::fs::metadata(path).ok()?;
    let modified = meta
        .modified()
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs();
    // 10-minute TTL (600 s), per the roadmap.
    if now.saturating_sub(modified) > 600 {
        return None;
    }
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn write_cache(path: &std::path::Path, value: &InfoValue) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    // Atomic write: temp file + rename (matches cache.rs).
    let temp = path.with_extension("json.tmp");
    if let Ok(json) = serde_json::to_string(value) {
        if std::fs::write(&temp, json).is_ok() {
            let _ = std::fs::rename(&temp, path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::symbol_emoji;

    #[test]
    fn symbol_emoji_maps_known_conditions() {
        assert_eq!(symbol_emoji("Sunny"), "☀️");
        assert_eq!(symbol_emoji("Clear"), "🌙");
        assert_eq!(symbol_emoji("Partly cloudy"), "🌤");
        assert_eq!(symbol_emoji("Light rain shower"), "🌧");
        assert_eq!(symbol_emoji("Mist"), "🌫");
        assert_eq!(symbol_emoji("Something unknown"), "🌡");
    }
}
