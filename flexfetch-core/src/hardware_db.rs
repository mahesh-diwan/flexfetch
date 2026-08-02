//! Crowdsourced hardware database (Phase 5.8).
//!
//! Identifies PCI/USB devices by vendor:device ID without embedding a huge
//! `pci.ids` table. A small seed ships with the binary; `flexfetch --update-db`
//! downloads the latest map to the cache dir (via curl — the same tool the
//! installer and `--update` rely on) and looks it up from there. When offline,
//! callers fall back to the seed, then to raw hex IDs.
//!
//! DB format (JSON, plain — no zstd to keep the minimal build dependency-free):
//! ```json
//! { "pci": { "10de:2684": "NVIDIA GeForce RTX 4090" },
//!   "usb": { "046d:c53f": "Logitech G Pro X" } }
//! ```

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

/// Seed database embedded at compile time (always available offline).
const SEED: &str = include_str!("../data/hardware.json");

/// The parsed seed map — parsed exactly once (Phase 4.1 zero-spawn philosophy:
/// `lookup` is called per GPU card, so re-parsing the seed JSON on every call
/// would add avoidable cold-start cost).
static SEED_DB: OnceLock<HashMap<String, String>> = OnceLock::new();

fn seed_db() -> &'static HashMap<String, String> {
    SEED_DB.get_or_init(|| parse_db(SEED))
}

/// Remote DB URL — the raw file on the flexfetch repo (works with plain curl,
/// no auth). Override with `FLEXFETCH_HWDB_URL` (e.g. a GitHub Pages mirror).
fn remote_url() -> String {
    std::env::var("FLEXFETCH_HWDB_URL").unwrap_or_else(|_| {
        "https://raw.githubusercontent.com/mahesh-diwan/flexfetch/main/packaging/hardware-db/hardware.json".to_string()
    })
}

/// The cached DB path (`~/.cache/flexfetch/hardware.json`).
pub fn cache_path() -> PathBuf {
    crate::get_cache_dir().join("hardware.json")
}

/// Parse the DB JSON into a `pci`/`usb` key → name map (unknown shape → empty).
fn parse_db(text: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let Ok(v) = serde_json::from_str::<serde_json::Value>(text) else {
        return map;
    };
    for section in ["pci", "usb"] {
        if let Some(obj) = v.get(section).and_then(|s| s.as_object()) {
            for (id, name) in obj {
                if let Some(name) = name.as_str() {
                    map.insert(id.to_lowercase(), name.to_string());
                }
            }
        }
    }
    map
}

/// Normalize a sysfs ID (e.g. `0x10de` / `10DE`) to the DB key form (`10de`).
fn normalize_id(raw: &str) -> String {
    raw.trim()
        .trim_start_matches("0x")
        .trim_start_matches("0X")
        .to_lowercase()
}

/// Look up a device name by vendor/device pair. Checks the cached DB first,
/// then the bundled seed; returns the canonical name or `None` (callers fall
/// back to raw hex / driver names). Vendor/device may carry a `0x` prefix.
pub fn lookup(vendor: &str, device: &str) -> Option<String> {
    let key = format!("{}:{}", normalize_id(vendor), normalize_id(device));

    if let Ok(cached) = std::fs::read_to_string(cache_path()) {
        if let Some(name) = parse_db(&cached).get(&key) {
            return Some(name.clone());
        }
    }
    seed_db().get(&key).cloned()
}

/// `--update-db`: download the latest DB into the cache dir. Uses `curl`
/// (consistent with `--update` / install.sh); fails with a clear error when
/// curl is missing or the fetch fails, so scripts can react.
pub fn refresh() -> crate::Result<String> {
    let url = remote_url();
    let mut cmd = std::process::Command::new("curl");
    cmd.args(["-fsSL", "--max-time", "20", &url]);
    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            return Err(crate::Error::Parse(format!(
                "curl unavailable ({e}) — install curl to refresh the hardware DB"
            )));
        }
    };
    if !output.status.success() {
        return Err(crate::Error::Parse(format!(
            "download failed (exit {:?}) from {url}",
            output.status.code()
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout).to_string();
    let count = parse_db(&text).len();
    if count == 0 {
        return Err(crate::Error::Parse(
            "downloaded DB contained no usable entries".into(),
        ));
    }
    let path = cache_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(crate::Error::Io)?;
    }
    std::fs::write(&path, &text).map_err(crate::Error::Io)?;
    Ok(format!(
        "hardware DB refreshed: {count} entries -> {}",
        path.display()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_has_known_ids() {
        // The bundled seed must resolve the GPU the repo's own CI/dev boxes use
        // (NVIDIA 10de / Intel 8086 / AMD 1002 vendors), so offline lookups work.
        let map = parse_db(SEED);
        assert!(!map.is_empty(), "seed DB must not be empty");
        assert!(map.contains_key("10de:2684") || map.contains_key("8086:9a49"));
    }

    #[test]
    fn normalize_strips_prefix_and_case() {
        assert_eq!(normalize_id("0x10DE"), "10de");
        assert_eq!(normalize_id("10de"), "10de");
        assert_eq!(normalize_id("  8086  "), "8086");
    }

    #[test]
    fn lookup_uses_seed_when_no_cache() {
        // Remove any stray cache file so the seed is the source of truth.
        let _ = std::fs::remove_file(cache_path());
        let map = parse_db(SEED);
        let first = map.keys().next().expect("seed non-empty").clone();
        let (v, d) = first.split_once(':').expect("id has colon");
        let name = lookup(v, d);
        assert!(name.is_some(), "{first} should resolve from the seed");
    }

    #[test]
    fn parse_db_rejects_garbage() {
        assert!(parse_db("not json").is_empty());
        assert!(parse_db("").is_empty());
    }
}
