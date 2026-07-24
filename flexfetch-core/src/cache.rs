use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Cache {
    path: PathBuf,
    ttl: u64,
    data: HashMap<String, CacheEntry>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CacheEntry {
    value: String,
    timestamp: u64,
}

impl Cache {
    pub fn new(cache_dir: PathBuf, ttl: u64) -> Self {
        let path = cache_dir.join("flexfetch-cache.json");
        let data = std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        Cache { path, ttl, data }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
        self.data.get(key).and_then(|entry| {
            if now - entry.timestamp < self.ttl {
                Some(entry.value.clone())
            } else {
                None
            }
        })
    }

    pub fn set(&mut self, key: &str, value: String) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        self.data.insert(
            key.to_string(),
            CacheEntry {
                value,
                timestamp: now,
            },
        );
        self.flush();
    }

    fn flush(&self) {
        if let Ok(json) = serde_json::to_string(&self.data) {
            // Create parent dir if missing
            if let Some(parent) = self.path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            // Atomic write: write to temp file then rename
            let temp_path = self.path.with_extension("json.tmp");
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                let result = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .mode(0o600)
                    .open(&temp_path)
                    .and_then(|mut f| {
                        use std::io::Write;
                        f.write_all(json.as_bytes())
                    });

                if result.is_ok() {
                    let _ = std::fs::rename(&temp_path, &self.path);
                }
            }

            #[cfg(not(unix))]
            {
                let result = std::fs::write(&temp_path, &json);
                if result.is_ok() {
                    let _ = std::fs::rename(&temp_path, &self.path);
                }
            }
        }
    }
}
