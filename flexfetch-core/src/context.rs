use crate::cache::Cache;
use crate::config::CustomModule;
use crate::fs::{FileSystem, RealFs};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct Context {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub debug: bool,
    /// `--stat`: record per-module collection timings into [`Context::timings`].
    pub stat: bool,
    /// Per-module collection timings in microseconds `(module, µs)`, recorded
    /// when `stat` is on. Wall-clock per module, including any timeout wait.
    pub timings: Mutex<Vec<(String, u64)>>,
    pub cache: Mutex<Cache>,
    pub custom_modules: HashMap<String, CustomModule>,
    fs: Box<dyn FileSystem>,
}

impl Context {
    pub fn new(
        config_dir: PathBuf,
        cache_dir: PathBuf,
        debug: bool,
        custom_modules: HashMap<String, CustomModule>,
    ) -> Self {
        Self::with_fs(
            config_dir,
            cache_dir,
            debug,
            custom_modules,
            Box::new(RealFs),
        )
    }

    /// Construct a context around an explicit [`FileSystem`] adapter (tests
    /// inject a `MockFs`; production uses `RealFs` via [`Context::new`]).
    pub fn with_fs(
        config_dir: PathBuf,
        cache_dir: PathBuf,
        debug: bool,
        custom_modules: HashMap<String, CustomModule>,
        fs: Box<dyn FileSystem>,
    ) -> Self {
        let cache = Cache::new(cache_dir.clone(), 60);
        Context {
            config_dir,
            cache_dir,
            debug,
            stat: false,
            timings: Mutex::new(Vec::new()),
            cache: Mutex::new(cache),
            custom_modules,
            fs,
        }
    }

    /// Read a file through the context abstraction. Modules should use this
    /// instead of std::fs::read_to_string to enable testing with mock data.
    pub fn read_file(&self, path: impl AsRef<Path>) -> Result<String, std::io::Error> {
        self.fs.read_to_string(path.as_ref())
    }

    /// List a directory's children through the context abstraction.
    pub fn read_dir(&self, path: impl AsRef<Path>) -> Result<Vec<PathBuf>, std::io::Error> {
        self.fs.read_dir(path.as_ref())
    }

    /// Whether a path exists (through the context abstraction).
    pub fn exists(&self, path: impl AsRef<Path>) -> bool {
        self.fs.exists(path.as_ref())
    }

    /// Whether a path is a directory (through the context abstraction).
    pub fn is_dir(&self, path: impl AsRef<Path>) -> bool {
        self.fs.is_dir(path.as_ref())
    }

    /// Update the cache TTL (seconds). Used to honor the config's `cache_ttl`
    /// key instead of the hardcoded 60 s default. Keeps already-loaded data.
    pub fn set_cache_ttl(&self, ttl_seconds: u64) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.set_ttl(ttl_seconds);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file_returns_content() {
        let dir = std::env::temp_dir().join(format!("ff-ctx-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.txt");
        std::fs::write(&path, "hello").unwrap();
        let ctx = Context::new(dir.clone(), dir.clone(), false, Default::default());
        assert_eq!(ctx.read_file(&path).unwrap(), "hello");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
