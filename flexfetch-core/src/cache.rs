use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// On-disk key/value cache (wifi, display, packages, bluetooth, media,
/// publicip results). The JSON file is read lazily on first `get`/`set`, so a
/// run whose selected modules never touch the cache performs zero file IO for
/// it — `Cache::new` itself is cheap.
pub struct Cache {
    path: PathBuf,
    ttl: u64,
    loaded: Cell<bool>,
    data: RefCell<HashMap<String, CacheEntry>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CacheEntry {
    value: String,
    timestamp: u64,
}

impl Cache {
    pub fn new(cache_dir: PathBuf, ttl: u64) -> Self {
        Cache {
            path: cache_dir.join("flexfetch-cache.json"),
            ttl,
            loaded: Cell::new(false),
            data: RefCell::new(HashMap::new()),
        }
    }

    /// Read the backing file once, on first use. No-op afterwards.
    fn ensure_loaded(&self) {
        if self.loaded.get() {
            return;
        }
        let data = std::fs::read_to_string(&self.path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        *self.data.borrow_mut() = data;
        self.loaded.set(true);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.ensure_loaded();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
        self.data.borrow().get(key).and_then(|entry| {
            // `saturating_sub`: a future timestamp (clock skew, corrupted file)
            // must not underflow — treat it as fresh rather than panicking.
            if now.saturating_sub(entry.timestamp) < self.ttl {
                Some(entry.value.clone())
            } else {
                None
            }
        })
    }

    /// Change the TTL (seconds) without discarding loaded entries.
    pub fn set_ttl(&mut self, ttl: u64) {
        self.ttl = ttl;
    }

    pub fn set(&mut self, key: &str, value: String) {
        // Load first so we don't clobber other modules' entries with a
        // partial map when this is the first access of the run.
        self.ensure_loaded();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.data.borrow_mut().insert(
            key.to_string(),
            CacheEntry {
                value,
                timestamp: now,
            },
        );
        self.flush();
    }

    fn flush(&self) {
        if let Ok(json) = serde_json::to_string(&*self.data.borrow()) {
            // Create parent dir if missing
            if let Some(parent) = self.path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            // Atomic write: write to a unique temp file then rename. The pid
            // suffix keeps concurrent flexfetch processes (e.g. a normal run
            // next to `--watch`) from clobbering each other's temp file.
            let temp_path = self
                .path
                .with_extension(format!("json.tmp.{}", std::process::id()));
            #[cfg(unix)]
            use std::os::unix::fs::OpenOptionsExt;
            let mut opts = std::fs::OpenOptions::new();
            opts.write(true).create(true).truncate(true);
            #[cfg(unix)]
            opts.mode(0o600);
            let result = opts.open(&temp_path).and_then(|mut f| {
                use std::io::Write;
                f.write_all(json.as_bytes())
            });
            if result.is_ok() {
                let _ = std::fs::rename(&temp_path, &self.path);
            }
        }
    }
}

pub fn get_cache_dir() -> PathBuf {
    std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".cache")
        })
        .join("flexfetch")
}

/// `~/.config/flexfetch` (or `$XDG_CONFIG_HOME/flexfetch`) — where
/// `config.toml`, `templates/*.tera`, and logo/image assets live.
pub fn get_config_dir() -> PathBuf {
    std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(home).join(".config")
        })
        .join("flexfetch")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("ff-cache-test-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// The file must not be read at construction: build a cache with a file
    /// present, then delete the file — a lazy cache keeps working from memory,
    /// an eager one would have loaded at construction either way. Instead we
    /// prove laziness the other direction: `get` before the file exists must
    /// return None, and `set` must not have loaded anything stale.
    #[test]
    fn construction_does_not_read_the_file() {
        let dir = temp_dir("lazy");
        // Pre-seed a file so an eager implementation would load it.
        std::fs::write(
            dir.join("flexfetch-cache.json"),
            r#"{"seed":{"value":"preloaded","timestamp":9999999999}}"#,
        )
        .unwrap();

        let cache = Cache::new(dir.clone(), 60);
        // Remove the file BEFORE any access. If construction had read it, the
        // seed entry would be in memory; a lazy cache never sees it.
        std::fs::remove_file(dir.join("flexfetch-cache.json")).unwrap();
        assert_eq!(
            cache.get("seed"),
            None,
            "lazy cache must not load at construction"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn get_reads_the_file_on_first_access() {
        let dir = temp_dir("first-access");
        std::fs::write(
            dir.join("flexfetch-cache.json"),
            r#"{"ip":{"value":"1.2.3.4","timestamp":9999999999}}"#,
        )
        .unwrap();
        let cache = Cache::new(dir.clone(), 60);
        assert_eq!(cache.get("ip").as_deref(), Some("1.2.3.4"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn expired_entries_return_none() {
        let dir = temp_dir("expired");
        std::fs::write(
            dir.join("flexfetch-cache.json"),
            r#"{"old":{"value":"stale","timestamp":1}}"#,
        )
        .unwrap();
        let cache = Cache::new(dir.clone(), 60);
        assert_eq!(
            cache.get("old"),
            None,
            "entry older than TTL must be ignored"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn set_persists_and_reloads() {
        let dir = temp_dir("persist");
        let mut cache = Cache::new(dir.clone(), 60);
        cache.set("ip", "9.9.9.9".into());
        drop(cache);

        let reloaded = Cache::new(dir.clone(), 60);
        assert_eq!(reloaded.get("ip").as_deref(), Some("9.9.9.9"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn set_does_not_clobber_existing_entries() {
        let dir = temp_dir("noclobber");
        std::fs::write(
            dir.join("flexfetch-cache.json"),
            r#"{"wifi":{"value":"ssid","timestamp":9999999999}}"#,
        )
        .unwrap();
        let mut cache = Cache::new(dir.clone(), 60);
        // First access is this `set` — it must load the file first so the
        // existing wifi entry survives the flush.
        cache.set("ip", "8.8.8.8".into());
        drop(cache);

        let reloaded = Cache::new(dir.clone(), 60);
        assert_eq!(
            reloaded.get("wifi").as_deref(),
            Some("ssid"),
            "set must load before writing"
        );
        assert_eq!(reloaded.get("ip").as_deref(), Some("8.8.8.8"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
